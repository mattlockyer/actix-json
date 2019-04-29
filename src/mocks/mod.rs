use std::fs::File;
use actix_web::{HttpResponse, Error};
use futures::{future::{ok}, Future};
use serde_json::{from_reader, Value as JSON};

pub fn data_test() -> impl Future<Item = HttpResponse, Error = Error> {
  let file = File::open("mocks/data_test.json");
  let data:JSON = from_reader(&file.unwrap()).unwrap();
  ok(HttpResponse::Ok().json(data))
}