use anyhow::Context;
use warp::{filters::BoxedFilter, Filter, Reply};

#[derive(rust_embed::RustEmbed)]
#[folder = "../../frontend/build"]
struct Asset;

pub fn asset_server() -> BoxedFilter<(Box<dyn Reply>,)> {
 static_filter()
  .or(favicon_filter())
  .or(catchall_filter())
  .map(|reply| Box::new(reply) as Box<dyn Reply>)
  .boxed()
}

fn static_filter() -> BoxedFilter<(Box<dyn Reply>,)> {
 warp::path("static")
  .and(warp::path::full())
  .map(|path: warp::path::FullPath| {
   match (|| -> anyhow::Result<_> {
    Ok(Box::new(warp::reply::with_header(
     std::str::from_utf8(
      Asset::get(path.as_str().trim_start_matches('/')).context("unwrap option")?.as_ref(),
     )?
     .to_string(),
     "content-type",
     "text/css",
    )) as Box<dyn Reply>)
   })() {
    Ok(body) => body,
    Err(_e) => panic!("panic!"), //warp::reply::html("error!".to_string()),
   }
  })
  .boxed()
}

fn favicon_filter() -> BoxedFilter<(Box<dyn Reply>,)> {
 warp::path("favicon.ico")
  .map(|| {
   Box::new(
    warp::http::Response::builder()
     .header("content-type", "image/x-icon")
     .body(Asset::get("favicon.ico").unwrap()),
   ) as Box<dyn Reply>
  })
  .boxed()
}

fn catchall_filter() -> BoxedFilter<(Box<dyn Reply>,)> {
 warp::any()
  .map(|| {
   Box::new(warp::reply::html(
    std::str::from_utf8(Asset::get("index.html").unwrap().as_ref()).unwrap().to_string(),
   )) as Box<dyn Reply>
  })
  .boxed()
}
