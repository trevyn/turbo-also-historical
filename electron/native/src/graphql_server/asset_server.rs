use warp::{filters::BoxedFilter, Filter, Reply};

#[derive(rust_embed::RustEmbed)]
#[folder = "../../frontend/build"]
struct Asset;

pub fn asset_server() -> BoxedFilter<(Box<dyn Reply>,)> {
 catchall_filter().map(|reply| Box::new(reply) as Box<dyn Reply>).boxed()
}

fn catchall_filter() -> BoxedFilter<(Box<dyn Reply>,)> {
 warp::path::full()
  .map(|path: warp::path::FullPath| {
   match (|| -> anyhow::Result<_> {
    let path = match path.as_str().trim_start_matches('/') {
     "" => "index.html",
     p => p,
    };
    let asset = match Asset::get(path) {
     Some(std::borrow::Cow::Borrowed(a)) => a,
     _ => panic!("could not load: {:#?}", path),
    };
    Ok(Box::new(warp::reply::with_header(
     asset,
     "content-type",
     mime_guess::from_path(path).first_raw().unwrap(),
    )) as Box<dyn Reply>)
   })() {
    Ok(body) => body,
    Err(_e) => panic!("could not load: {:#?}", path), //warp::reply::html("error!".to_string()),
   }
  })
  .boxed()
}
