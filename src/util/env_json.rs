/********************************

Create a file in root (not src)

// env.json
{
  "postgres":{
    "username": "xxxxx",
    "password": "xxxxx"
  }
}

********************************/

use serde_derive::{Deserialize, Serialize};
use serde_json::{ from_value, };
use crate::{json_open};

#[derive(Serialize, Deserialize, Debug)]
pub struct Env {
  pub postgres: Postgres,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Postgres {
  pub username: String,
  pub password: String,
}

pub fn init() -> Env {
  from_value(json_open("env.json")).unwrap()
}