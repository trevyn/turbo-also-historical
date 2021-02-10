use neon::event::EventHandler;
use neon::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[allow(clippy::type_complexity)]
static PROSEMIRROR_CALLBACK: Lazy<Mutex<Option<Box<dyn Fn() + Send>>>> =
 Lazy::new(|| Mutex::new(None));

#[allow(clippy::unnecessary_wraps)]
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
 if let Some(c) = PROSEMIRROR_CALLBACK.lock().unwrap().as_ref() {
  eprintln!("calling c()");
  c();
 } else {
  eprintln!("c not ready yet!");
 };

 Ok(cx.string("hello node 2"))
}

fn register_prosemirror_apply_callback(mut cx: FunctionContext) -> JsResult<JsUndefined> {
 let undefined = cx.undefined();

 let this = cx.this();
 let func = cx.argument::<JsFunction>(0)?;
 let cb = EventHandler::new(&cx, this, func);

 let execute = move || {
  cb.schedule(move |cx| {
   eprintln!("in scheduled");
   let args: Vec<Handle<JsValue>> =
    vec![cx.string("progress").upcast(), cx.string("1".to_string()).upcast()];
   args
  })
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
