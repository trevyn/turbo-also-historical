pub fn apply_steps(document: &str, steps: &str) -> anyhow::Result<String> {
 dbg!(document, steps);

 // let mut runtime = JsRuntime::new(Default::default());

 // runtime.register_op("op_print", |_state, zero_copy| {
 //  let mut out = std::io::stdout();
 //  for buf in zero_copy {
 //   out.write_all(&buf).unwrap();
 //  }

 //  Op::Sync(Box::new([])) // No meaningful result
 // });

 // runtime.execute(
 //  "build/prosemirror-collab-server.js",
 //  include_str!("../build/prosemirror-collab-server.js"),
 // )?;

 Ok("done!".to_string())
}
