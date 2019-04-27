use actix_web::{web, HttpResponse, Error};
use futures::{future::{ok}, Future};
use serde_json::{json, Value as JSON};

/// 404 handler
pub fn not_found() -> impl Future<Item = HttpResponse, Error = Error> {
  let res = json!({
    "success": false,
    "message": "404 Not Found",
  });
  ok(HttpResponse::NotFound().json(res))
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