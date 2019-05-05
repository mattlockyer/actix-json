
use actix_web::{web, HttpRequest, HttpResponse, Error, http::{header},};
use futures::{future::{ok}, Future};
use serde_json::{json, Value as JSON};

use crate::async_res;
use super::{json_res, json_res_msg};

async_res!(get_test, { json_res_msg(200, true, "get_test") });

async_res!(post_test, body, {
  json_res(200, json!({
    "success": true,
    "body": {
      "type": "json",
      "payload": JSON::as_object(&body),
    },
  }))
});

// json post test
// pub fn post_test(body: web::Json<JSON>) -> impl Future<Item = HttpResponse, Error = Error> {
//   let res = json!({
//     "success": true,
//     "body": {
//       "type": "json",
//       "payload": JSON::as_object(&body),
//     },
//   });
//   ok(HttpResponse::Ok().json(res))
// }

//redirect
pub fn redirect(req:HttpRequest, to:&str) -> HttpResponse {
  println!("{:?} redirected to {}", req, to);
  HttpResponse::Found().header(header::LOCATION, to).finish()
}