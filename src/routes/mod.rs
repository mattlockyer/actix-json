
use actix_web::{
  HttpRequest, HttpResponse, Error, 
  http::{header}, web::{Json},
};
use futures::{future::{ok}, Future};
use serde_json::{json, Value as JSON};

use crate::future_ok;
use super::{json_res, json_msg};

future_ok!(get_test, { json_msg(200, true, "get_test") });

future_ok!(
  post_test,
  {
    json_res(200, json!({
      "success": true,
      "body": {
        "type": "json",
        "payload": JSON::as_object(&body),
      },
    }))
  },
  body, Json<JSON>
);

//redirect
pub fn redirect(req:HttpRequest, to:&str) -> HttpResponse {
  println!("{:?} redirected to {}", req, to);
  HttpResponse::Found().header(header::LOCATION, to).finish()
}