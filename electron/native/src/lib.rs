use d_macro::*;
use neon::{event::EventHandler, prelude::*};
use once_cell::sync::Lazy;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Mutex, task::Poll};
use turbo_server::ApplyStepsFn;

static PROSEMIRROR_CALLBACK: Lazy<Mutex<Option<ApplyStepsFn>>> = Lazy::new(|| Mutex::new(None));

static RESULTWAKERS: Lazy<Mutex<HashMap<u8, ResultWaker>>> =
 Lazy::new(|| Mutex::new(HashMap::new()));

#[allow(clippy::unnecessary_wraps)]
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
 if let Some(_c) = PROSEMIRROR_CALLBACK.lock().unwrap().as_ref() {
  eprintln!("c is ready!");
 // async { c("10".to_string(), "20".to_string()).await };
 } else {
  eprintln!("c not ready yet!");
 };

 Ok(cx.string("hello node 2"))
}

async fn _do_thing() {
 eprintln!("do thing");
 null_fut().await;
}

fn null_fut() -> Pin<Box<dyn Future<Output = String> + Send + Sync>> {
 Box::pin(MyFut { slot: 0 })
}

fn helper(mut cx: FunctionContext) -> JsResult<JsUndefined> {
 let slot = cx.argument::<JsNumber>(0)?.value() as u8;
 let result = cx.argument::<JsString>(1)?.value();

 d!(#? (slot, &result));

 let mut resultwakers = RESULTWAKERS.lock().unwrap();
 let resultwaker = resultwakers.get_mut(&slot).unwrap();

 resultwaker.result = Some(result);
 if let Some(waker) = resultwaker.waker.take() {
  waker.wake();
 }

 Ok(cx.undefined())
}

#[derive(Debug)]
struct ResultWaker {
 result: Option<String>,
 waker: Option<std::task::Waker>,
}

struct MyFut {
 slot: u8,
}

impl Future for MyFut {
 type Output = String;

 fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context) -> Poll<Self::Output> {
  let slot = &self.slot;
  let mut resultwakers = RESULTWAKERS.lock().unwrap();
  let resultwaker = resultwakers.get_mut(slot).unwrap();

  match resultwaker.result.take() {
   None => {
    resultwaker.waker = Some(cx.waker().clone());
    Poll::Pending
   }
   Some(result) => {
    d!(#? ("removing", slot, &result));
    resultwakers.remove(slot);
    Poll::Ready(result)
   }
  }
 }
}

fn register_prosemirror_apply_callback(mut cx: FunctionContext) -> JsResult<JsUndefined> {
 let this = cx.this();
 let func = cx.argument::<JsFunction>(0)?;
 let eventhandler = EventHandler::new(&cx, this, func);

 let execute =
  move |old: String, patch: String| -> Pin<Box<dyn Future<Output = String> + Send + Sync>> {
   let slot = {
    let mut resultwakers = RESULTWAKERS.lock().unwrap();
    let slot = (1..200).find(|n| !resultwakers.contains_key(n)).unwrap();
    assert!(resultwakers.insert(slot, ResultWaker { result: None, waker: None }).is_none());
    slot
   };

   let fut = MyFut { slot };

   eventhandler.schedule(move |cx: &mut neon::context::TaskContext| {
    eprintln!("in scheduled");

    let helper = neon::types::JsFunction::new(cx, helper).unwrap();

    let args: Vec<Handle<JsValue>> = vec![
     helper.upcast(),
     cx.number(slot).upcast(),
     cx.string(old).upcast(),
     cx.string(patch).upcast(),
    ];

    args
   });

   Box::pin(fut)
  };

 *PROSEMIRROR_CALLBACK.lock().unwrap() = Some(Box::new(execute));
 Ok(cx.undefined())
}

fn rust_log(mut cx: FunctionContext) -> JsResult<JsUndefined> {
 let s = cx.argument::<JsString>(0)?.value();
 log::info!("rust_log: {:#?}", s);
 turbo_server::rust_log(s);
 Ok(cx.undefined())
}

#[neon::main]
fn my_module(mut cx: ModuleContext) -> NeonResult<()> {
 log::debug!("neon module init");
 cx.export_function("hello", hello)?;
 cx.export_function("rustLog", rust_log)?;
 cx.export_function("beforeQuit", before_quit)?;
 cx.export_function("registerProsemirrorApplyCallback", register_prosemirror_apply_callback)?;

 let apply_steps =
  |old: String, patch: String| -> Pin<Box<dyn Future<Output = String> + Send + Sync>> {
   if let Some(c) = PROSEMIRROR_CALLBACK.lock().unwrap().as_ref() {
    c(old, patch)
   } else {
    null_fut()
   }
  };

 // launch the server
 std::thread::spawn(move || {
  turbo_server::run(Box::new(apply_steps));
 });

 Ok(())
}

/// `before-quit` event from Electron
#[allow(clippy::unnecessary_wraps)]
fn before_quit(mut cx: FunctionContext) -> JsResult<JsUndefined> {
 turbo_server::shutdown().unwrap();
 Ok(cx.undefined())
}
