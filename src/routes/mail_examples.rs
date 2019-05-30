
use actix_web::{ 
  web::{ Data },
  HttpResponse, Error 
};
use futures::{Future};
use crate::{Mailgun, fres};

pub fn mail_test(mailgun:Data<Mailgun>) -> fres!() {
  mailgun.send("mattdlockyer@gmail.com".to_string(), "TESTING".to_string(), "TEST".to_string())
}
