#[allow(unused_imports)]
use actix_web::{HttpResponse, Error, web::{Json}};
#[allow(unused_imports)]
use futures::{future::{ok}, Future};
#[allow(unused_imports)]
use serde_json::{Value as JSON};

#[macro_export]
macro_rules! async_res {
  ($name:ident, $code:block) => (
    pub fn $name() -> impl Future<Item = HttpResponse, Error = Error> {
      ok($code)
    }
  );
  ($name:ident, $arg: ident, $code:block) => (
    pub fn $name($arg:web::Json<JSON>) -> impl Future<Item = HttpResponse, Error = Error> {
      ok($code)
    }
  );
}