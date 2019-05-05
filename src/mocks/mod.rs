use std::{fs::File, io::Write};
use actix_web::{ web::{Path, Json}, HttpResponse, Error };
use futures::{Future, future::{ok}};
use serde_json::{from_reader, Value as JSON, error};

// util mod is a sibling module
use super::{json_res, json_res_msg};

pub fn mock_get(filename: Path<String>) -> impl Future<Item = HttpResponse, Error = Error> {
  //println!("path {:?}", filename.unwrap());
  let file = File::open(format!("{}{}", "mock/", filename));
  //println!("file {:?}", file);
  if !file.is_ok() {
    return ok(json_res_msg(500, false, "file not found"));
  }
  let data:Result<JSON, error::Error> = from_reader(&file.unwrap());
  if !data.is_ok() {
    return ok(json_res_msg(500, false, "no data found in file"));
  }
  ok(json_res(200, data.unwrap()))
}

pub fn mock_set(filename: Path<String>, body: Json<JSON>) -> impl Future<Item = HttpResponse, Error = Error> {
  //println!("data {:?}", body.to_string());
  let file = File::create(format!("{}{}", "mock/", filename));
  //println!("file {:?}", file);
  if !file.is_ok() {
    return ok(json_res_msg(500, false, "error creating file"));
  }
  let result = file.unwrap().write_all(body.to_string().as_bytes());
  if !result.is_ok() {
    return ok(json_res_msg(500, false, "error writing file"));
  }
  ok(json_res_msg(200, true, "file written"))
}
