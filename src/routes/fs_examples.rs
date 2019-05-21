use std::{fs::File, io::Write};
use actix_web::{ web::{Path, Json}, HttpResponse, Error };
use futures::{Future, future::{ok}};
use serde_json::{Value as JSON};

use crate::future_ok;
// util mod is a sibling module
use crate::{json_res, json_msg, json_open};

future_ok!(
  mock_get,
  {
    json_res(200, json_open(&format!("{}{}", "mock/", filename)))
  },
  filename:Path<String>
);

future_ok!(
  mock_set,
  {
    //println!("data {:?}", body.to_string());
    let file = File::create(format!("{}{}", "mock/", filename));
    //println!("file {:?}", file);
    if file.is_ok() {
      return json_msg(500, false, "error creating file");
    }
    let result = file.unwrap().write_all(body.to_string().as_bytes());
    if !result.is_ok() {
      return json_msg(500, false, "error writing file");
    }
    json_msg(200, true, "file written")
  },
  filename:Path<String>, body:Json<JSON>
);
