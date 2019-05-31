

use actix_web::{ 
  client::Client,
  web::{ Data, Query,},
  HttpResponse, Error
};
use futures::{Future};
use serde_derive::{Deserialize};
use serde_json::{from_slice, Value as JSON};
use crate::{fres};

#[derive(Debug, Deserialize)]
pub struct Info {
  url: String,
}
/********************************
Incomplete, should take ? get param to use URLs
********************************/
pub fn proxy_streaming(info: Query<Info>, client:Data<Client>) -> fres!() {
  println!("{:?}", info);
  client
    .get(&info.url)
    .header("User-Agent", "Actix-web")
    .send()
    .map_err(Error::from)
    .and_then(|res| {
      Ok(HttpResponse::Ok().streaming(res))
    })
}

/********************************
Working
********************************/

pub fn proxy_test(client:Data<Client>) -> fres!() {
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


