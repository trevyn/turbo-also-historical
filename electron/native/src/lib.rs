// use futures::prelude::*;
use neon::event::EventHandler;
use neon::prelude::*;
use once_cell::sync::Lazy;
use std::{
 collections::HashMap,
 future::Future,
 pin::Pin,
 sync::Mutex,
 task::{Poll, Waker},
};

#[allow(clippy::type_complexity)]
static PROSEMIRROR_CALLBACK: Lazy<
 Mutex<Option<Box<dyn Fn(String, String) -> Pin<Box<dyn Future<Output = String>>> + Send>>>,
> = Lazy::new(|| Mutex::new(None));

static RESULTWAKERS: Lazy<Mutex<HashMap<i32, ResultWakers>>> =
 Lazy::new(|| Mutex::new(HashMap::new()));

#[allow(clippy::unnecessary_wraps)]
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
 if let Some(c) = PROSEMIRROR_CALLBACK.lock().unwrap().as_ref() {
  eprintln!("calling c()");
  c("10".to_string(), "20".to_string());
 } else {
  eprintln!("c not ready yet!");
 };

 Ok(cx.string("hello node 2"))
}

async fn do_thing() {
 eprintln!("do thing");
 my_fut().await;
}

fn my_fut() -> Pin<Box<dyn Future<Output = String>>> {
 Box::pin(MyFut { slot: 0 })
}

#[allow(clippy::unnecessary_wraps)]
fn helper(mut cx: FunctionContext) -> JsResult<JsUndefined> {
 Ok(cx.undefined())
}

#[derive(Debug)]
struct ResultWakers {
 result: Option<String>,
 waker: Option<std::task::Waker>,
}

struct MyFut {
 slot: i32,
 // result: Option<String>,
 // waker: Option<std::task::Waker>,
 // closure: Option<Box<dyn FnMut(String) + Send>>,
}

impl Future for MyFut {
 type Output = String;
 fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context) -> Poll<Self::Output> {
  let mut resultwakers = RESULTWAKERS.lock().unwrap();
  let r = resultwakers.get_mut(&self.slot).unwrap().result.take();

  match r {
   None => {
    let waker: Waker = cx.waker().clone();
    resultwakers.get_mut(&self.slot).unwrap().waker = Some(waker);
    Poll::Pending
   }
   Some(r) => {
    resultwakers.remove(&self.slot);
    Poll::Ready(r)
   }
  }
 }
}

fn register_prosemirror_apply_callback(mut cx: FunctionContext) -> JsResult<JsUndefined> {
 let undefined = cx.undefined();

 let this = cx.this();
 let func = cx.argument::<JsFunction>(0)?;
 let eventhandler = EventHandler::new(&cx, this, func);

 let execute = move |old: String, patch: String| -> Pin<Box<dyn Future<Output = String>>> {
  let slot = {
   let mut resultwakers = RESULTWAKERS.lock().unwrap();
   let slot = (1..100).find(|n| !resultwakers.contains_key(n)).unwrap();
   assert!(resultwakers.insert(slot, ResultWakers { result: None, waker: None }).is_none());
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
 Ok(undefined)
}

fn rust_log(mut cx: FunctionContext) -> JsResult<JsUndefined> {
 let s = cx.argument::<JsString>(0)?.value();
 eprintln!("rust_log: {:#?}", s);
 turbo_server::rust_log(s);
 Ok(cx.undefined())
}

#[neon::main]
fn my_module(mut cx: ModuleContext) -> NeonResult<()> {
 log::debug!("neon module init");
 cx.export_function("hello", hello)?;
 cx.export_function("rustLog", rust_log)?;
 cx.export_function("registerProsemirrorApplyCallback", register_prosemirror_apply_callback)?;

 // launch the server
 std::thread::spawn(|| {
  turbo_server::run();
 });

 Ok(())
}
