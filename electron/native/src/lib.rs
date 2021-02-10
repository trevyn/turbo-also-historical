use futures::prelude::*;
use neon::event::EventHandler;
use neon::prelude::*;
use once_cell::sync::Lazy;
use std::{future::Future, pin::Pin, sync::Mutex, task::Poll};

#[allow(clippy::type_complexity)]
static PROSEMIRROR_CALLBACK: Lazy<
 Mutex<Option<Box<dyn Fn(String, String) -> Pin<Box<dyn Future<Output = String>>> + Send>>>,
> = Lazy::new(|| Mutex::new(None));

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
}

// fn helper(cx: FunctionContext) -> JsResult<JsUndefined> {
//  Ok(cx.undefined())
// }

struct MyFut;

impl Future for MyFut {
 type Output = String;
 fn poll(self: Pin<&mut Self>, cx: &mut futures::task::Context) -> Poll<Self::Output> {
  Poll::Ready("42".to_string())
 }
}

fn my_fut() -> Pin<Box<dyn Future<Output = String>>> {
 Box::pin(MyFut)
}

fn register_prosemirror_apply_callback(mut cx: FunctionContext) -> JsResult<JsUndefined> {
 let undefined = cx.undefined();

 let this = cx.this();
 let func = cx.argument::<JsFunction>(0)?;
 let cb = EventHandler::new(&cx, this, func);

 let execute = move |old: String, patch: String| -> Pin<Box<dyn Future<Output = String>>> {
  // let mut output: Option<String> = None;

  cb.schedule(move |cx| {
   eprintln!("in scheduled");

   // let success = neon::types::JsFunction::new(&mut cx, helper);

   let args: Vec<Handle<JsValue>> = vec![cx.string(old).upcast(), cx.string(patch).upcast()];
   args
  });

  Box::pin(MyFut)
 };

 let mut cb_ref = PROSEMIRROR_CALLBACK.lock().unwrap();
 *cb_ref = Some(Box::new(execute));

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
