use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
 Ok(cx.string("hello node 2"))
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

 // launch the server
 std::thread::spawn(|| {
  turbo_server::run();
 });

 Ok(())
}
