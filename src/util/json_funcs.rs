// use actix_web::{ HttpResponse };
// use serde_json::{json, Value as JSON};

use std::{fs::File};
use actix_web::{ HttpResponse };
use serde_json::{json, from_reader, Value as JSON, error};

/********************************
Helpers -> HttpResponse
********************************/
pub fn json_msg(num:u16, success:bool, message:&str) -> HttpResponse {
  json_res(num, json_msg_raw(success, message))
}

pub fn json_res(num:u16, payload:JSON) -> HttpResponse {
  return match num {
    200 => HttpResponse::Ok().json(payload),
    400 => HttpResponse::BadRequest().json(payload),
    403 => HttpResponse::Forbidden().json(payload),
    404 => HttpResponse::NotFound().json(payload),
    500 => HttpResponse::InternalServerError().json(payload),
    501 => HttpResponse::NotImplemented().json(payload),
    _ => HttpResponse::InternalServerError().json(payload)
  }
}

/********************************
Helpers -> JSON
********************************/
pub fn json_open(path: &str) -> JSON {
  //println!("path {:?}", filename.unwrap());
  let file = File::open(path);
  //println!("file {:?}", file);
  if !file.is_ok() {
    return json_msg_raw(false, "file not found");
  }
  let data:Result<JSON, error::Error> = from_reader(&file.unwrap());
  if !data.is_ok() {
    return json_msg_raw(false, "no data found in file");
  }
  data.unwrap()
}

pub fn json_msg_raw(success:bool, message:&str) -> JSON {
  json!({
    "success": success,
    "message": message
  })
}