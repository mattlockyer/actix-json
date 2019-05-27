

use actix_web::client::Client;
use actix_web::{ web::{Path}, HttpResponse, Error};
use futures::{Future};

use crate::{future};
use serde_json::{from_slice, Value as JSON};

/********************************
Incomplete, should take ? get param to use URLs
********************************/
future!(proxy_streaming,
  {
    let client:Client = Client::new();
    client
      .get(&format!("http://{}", url))
      .header("User-Agent", "Actix-web")
      .send()
      .map_err(Error::from)
      .and_then(|res| {
        Ok(HttpResponse::Ok().streaming(res))
      })
  },
  url:Path<String>
);

/********************************
Working
********************************/
future!(proxy_test,
  {
    let client:Client = Client::new();
    client
      .get("http://localhost:8080/get_test")
      .header("User-Agent", "Actix-web")
      .send()
      .map_err(Error::from)
      .and_then(|mut res| {
        res.body()
        .map_err(Error::from)
        .and_then(|body| {
          println!("==== BODY ====");
          println!("{:?}", body);
          let json:JSON = from_slice(&body).unwrap();
          Ok(HttpResponse::Ok().json(json))
        })
      })
  }
);


