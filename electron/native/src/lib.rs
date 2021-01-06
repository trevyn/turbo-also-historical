use neon::prelude::*;

mod graphql_server;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
 Ok(cx.string("hello node 2"))
}

// register_module!(mut cx, {
//     eprintln!("neon module init 2");
//     cx.export_function("hello", hello)?;
//     Ok(())
// });

// @mark neon
#[neon::main]
fn my_module(mut cx: ModuleContext) -> NeonResult<()> {
 eprintln!("neon module init");
 cx.export_function("hello", hello)?;
 // let version = cx.string("1.0.0");
 // cx.export_value("version", version)?;

 // launch the server
 std::thread::spawn(|| {
  graphql_server::run();
 });

 Ok(())
}
