
use actix_web::client::Client;
use actix_web::{HttpResponse, Error};
use futures::{Future};
use serde_derive::{Serialize};
use serde_json::{from_slice, Value as JSON};
use crate::{fres};

#[derive(Clone)]
pub struct Mailgun {
  domain: String,
  apikey: String,
}

#[derive(Serialize)]
struct MailGunData {
  from: String,
  to: String,
  subject: String,
  text: String,
}

impl Mailgun {

  pub fn send(&self, to:String, subject:String, text:String) -> fres!() {
    let data = MailGunData{
      from: String::from(format!("Test <test@{}>", self.domain)),
      to,
      subject,
      text,
    };
    
    Client::default()
      .post(format!("https://api.mailgun.net/v3/{}/messages", self.domain))
      .basic_auth("api", Some(&self.apikey))
      .send_form(&data)
      .map_err(Error::from)
      .and_then(|mut res| {
        res.body()
        .map_err(Error::from)
        .and_then(|body| {
          println!("===== RESPONSE BODY =====");
          println!("{:?}", body);
          let json:JSON = from_slice(&body).unwrap();
          Ok(HttpResponse::Ok().json(json))
        })
      })
  }
}

pub fn init(domain: String, apikey: String) -> Mailgun {
  let mailgun = Mailgun {
    domain: domain.clone(),
    apikey: apikey.clone(),
  };
  return mailgun;
}