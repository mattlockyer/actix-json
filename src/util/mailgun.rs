
use actix_web::client::Client;
use actix_web::{HttpResponse, Error};
use futures::{future::{ok}, Future};


use serde_json::{from_slice, Value as JSON};

use crate::{fres};


#[derive(Clone)]
pub struct Mailgun {
  client: Client,
  domain: String,
  apikey: String,
}

unsafe impl Send for Mailgun {}

impl Mailgun {
  pub fn test(&self) -> fres!() {
    println!("sending request");
    self.client
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
}

pub fn init(domain: String, apikey: String) -> Mailgun {
  let mailgun = Mailgun {
    client: Client::new(),
    domain: domain.clone(),
    apikey: apikey.clone(),
  };
  return mailgun;
}