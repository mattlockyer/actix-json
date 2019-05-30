use actix_web::{ 
  client::Client,
  web::{ Data },
  HttpResponse, Error 
};
use futures::{Future};
use serde_derive::{Serialize};
use serde_json::{from_slice, Value as JSON};

use crate::{fres};

/********************************
Testing memory stuff

Panics essentially leak the thread and it doesn't get reclaimed
TLDR: handle all results, never trust data formats / .unwrap()
unless catching and sending response to client
********************************/

#[derive(Serialize)]
struct TestUserData {
  name: String,
  salary: u32,
  age: u8,
}


pub fn api_test(client:Data<Client>) -> fres!() {
  let data = TestUserData{
    name: String::from("Matt"),
    salary: 120000,
    age: 34,
  };

// force a panic for testing worker thread leaks
// let json:JSON = from_str("NOT JSON").unwrap();
// println!("{:?}", json);
// ok(HttpResponse::Ok().json(json))
  
  client
    .post("http://dummy.restapiexample.com/api/v1/create")
    .send_json(&data)
    .map_err(Error::from)
    .and_then(|mut res| {
      res.body()
      .map_err(Error::from)
      .and_then(|body| {
        println!("===== RESPONSE BODY =====");
        println!("{:?}", body);

        // this will cause a panic if body is NOT valid JSON
        //let json:JSON = from_slice(&body).unwrap();

        // ugly match syntax
        // let json:JSON = match from_slice(&body) {
        //   Err(why) => json_msg_raw(false, &format!("{:?}", why)),
        //   Ok(json) => json,
        // };

        // note the ? this will handle the error instead of ugly match syntax
        let json:JSON = from_slice(&body)?;

        Ok(HttpResponse::Ok().json(json))
      })
    })
}