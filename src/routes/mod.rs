
use actix_web::{web, HttpRequest, HttpResponse, Error, http::{header},};
use futures::{future::{ok}, Future};
use serde_json::{json, Value as JSON};

//redirect
pub fn redirect(req:HttpRequest, to:&str) -> HttpResponse {
  println!("{:?} redirected to {}", req, to);
  HttpResponse::Found().header(header::LOCATION, to).finish()
}
// json get test
pub fn get_test() -> impl Future<Item = HttpResponse, Error = Error> {
  let res = json!({
    "success": true,
  });
  ok(HttpResponse::Ok().json(res))
}

// json post test
pub fn post_test(body: web::Json<JSON>) -> impl Future<Item = HttpResponse, Error = Error> {
  let res = json!({
    "success": true,
    "body": {
      "type": "json",
      "payload": JSON::as_object(&body),
    },
  });
  ok(HttpResponse::Ok().json(res))
}