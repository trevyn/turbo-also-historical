use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
 Ok(cx.string("hello node 2"))
}

fn register_prosemirror_apply_callback(mut cx: FunctionContext) -> JsResult<JsString> {
 let this = cx.this();
 let func = cx.argument::<JsFunction>(0)?;
 let cb = neon::event::EventHandler::new(&cx, this, func);
 std::thread::spawn(move || {
  for i in 0..100 {
   // do some work ....
   std::thread::sleep(std::time::Duration::from_millis(40));
   // schedule a call into javascript
   cb.schedule(move |cx| {
    // return the arguments of the function call
    let args: Vec<Handle<JsValue>> =
     vec![cx.string("progress").upcast(), cx.string(i.to_string()).upcast()];
    args
   })
  }
 });

 Ok(cx.string("hello register_prosemirror_apply_callback"))
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
