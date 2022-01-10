#![deny(warnings)]

use std::path::PathBuf;
use std::{env};
use mime_guess;
use hyper::header;
// use handlebars::{Handlebars, RenderError, RenderContext, Helper, JsonRender};

use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};

static NOTFOUND: &[u8] = b"404 File Not Found";

/// HTTP status code 404ll
fn not_found() -> Response<Body> {
  Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(NOTFOUND.into())
      .unwrap()
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  // let mut current_exec = env::current_exe().unwrap();
  let mut current_woker = env::current_dir().unwrap();

  let origin_url_path = PathBuf::from(req.uri().path());
  // req.uri()
  let request_path = PathBuf::from(req.uri().path());
  let request_path_str = request_path.strip_prefix("/").unwrap().display().to_string();
  // let current_exec_pb = PathBuf.from(current_exec);
  current_woker.push(request_path_str);
  
  println!("current request path: -> {}", origin_url_path.as_path().display().to_string());

  // current_exec.as_path().to_str()
  let abs_path = current_woker.as_path().display().to_string();

  println!("read local path: -> {}", abs_path);

  let current_mime = mime_guess::from_path(current_woker.as_path());

  let mut current_mime_str = String::from("");

  if !current_mime.first().is_none() {
    current_mime_str = current_mime.first().unwrap().to_string();
  }

  if current_woker.is_dir() {
    println!("current request is dir: -> {:#}", current_woker.as_path().display());
    let origin_str = origin_url_path.as_path().display().to_string();
    // 如果不是以/结尾尝试302跳转
    if !origin_str.ends_with("/") {
      let origin_url_path_str = origin_url_path.as_path().display().to_string() + "/";
      println!("try redirect with /: -> {:#}", origin_url_path_str);
      return Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, origin_url_path_str)
        .body(Body::from(""))
        .unwrap())
    }
    // try index.html
    current_woker.push("index.html")
  } else {
    println!("try read list: -> {:#}", current_woker.as_path().display());
  }

  if current_woker.exists() {
      let final_abs_path = current_woker.as_path().display().to_string();
      println!("request: -> {}", final_abs_path);
      if let Ok(file) = File::open(final_abs_path).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(
          Response::builder()
          .header(header::CONTENT_TYPE, current_mime_str)
          .body(body)
          .unwrap()
        );
      } else {
        Ok(
          Response::new(Body::from("request failed"))
        )
      }
    } else {
      Ok(not_found())
    }

  // assert_eq!(request_path, "/");
  
  // Ok(Response::new(Body::from("hello")))
}

#[tokio::main]
async fn main() {
  let addr = ([127, 0, 0, 1], 3000).into();
  let current_path = env::current_dir().unwrap();
  let current_exec = env::current_exe().unwrap();

  assert_eq!(current_path.to_str(), Some("/Users/liepin/project/hyper"));

  println!("Hello, world! {:?}", env::current_dir().unwrap().display());
  println!("Hello, world! {}", current_exec.display());

  let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

  let server = Server::bind(&addr).serve(service);

  println!("Listening on http://{}", addr);

  // server.await?;
  // Ok(())
  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  } 
  // else {
  //   println!("server success.");
  // }
}
