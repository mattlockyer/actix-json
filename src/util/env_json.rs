/********************************
// env.json (in root not /src)
{
  "postgres": {
    "username": "xxxxx",
    "password": "xxxxx"
  },
  "mailgun": {
    "domain": "xxxxx",
    "apikey": "xxxxx",
  }
}
********************************/
use serde_derive::{Deserialize, Serialize};
use serde_json::{ from_value, };
use crate::{json_open};

#[derive(Serialize, Deserialize, Debug)]
pub struct Env {
  pub postgres: Postgres,
  pub mailgun: Mailgun,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Postgres {
  pub username: String,
  pub password: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Mailgun {
  pub domain: String,
  pub apikey: String,
}

pub fn init() -> Env {
  from_value(json_open("env.json")).unwrap()
}