use actix_web::{
  HttpRequest, HttpResponse, Error, 
  http::{header}, web::{Json},
};
use futures::{future::{ok}, Future};
use serde_json::{json, Value as JSON};

use crate::{fres, json_res, json_msg};

pub fn get_test() -> fres!() {
  ok(json_msg(200, true, "get_test"))
}

pub fn post_test(body:Json<JSON>) -> fres!() {
  ok(json_res(200, json!({
    "success": true,
    "body": {
      "type": "json",
      "payload": JSON::as_object(&body),
    },
  })))
}

//redirect
pub fn redirect(req:HttpRequest, to:&str) -> HttpResponse {
  println!("{:?} redirected to {}", req, to);
  HttpResponse::Found().header(header::LOCATION, to).finish()
}