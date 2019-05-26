
use actix_web::{ 
  web::{ Data, Json, },
  HttpResponse, Error 
};
use futures::{future::{ok}, Future};
//use serde_json::{json, Value as JSON};

use crate::{Mailgun, fres, json_msg};

pub fn mail_test(mailgun:Data<Mailgun>) -> fres!() {

  mailgun.test()
  //Ok(HttpResponse::Ok().streaming())

}

//test route
//future_ok!(mail_test, { json_msg(200, true, "mail_test") });
// future_ok!(
//   mail_test,
//   {
//     mailgun.test();
//     let two = time::Duration::from_millis(2000);
//     thread::sleep(two);
//     json_msg(200, true, "mail_test")
//   },
//   mailgun:Data<Mailgun>
// );
