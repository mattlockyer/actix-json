#[allow(unused_imports)]
use actix_web::{HttpResponse, Error};
#[allow(unused_imports)]
use futures::{future::{ok}, Future};

// gen a public function that returns future::ok, but matches actix to_async type req.
// params: $name, $code (executed as closure), multiple $arg:$type
#[macro_export]
macro_rules! future_ok {

  ($name:ident, $code:expr) => (
    pub fn $name() -> impl Future<Item = HttpResponse, Error = Error> {
      ok((||$code)())
    }
  );

  ($name:ident, $code:expr, $($arg:ident:$type:ty),*) => (
    pub fn $name($($arg:$type),*,) -> impl Future<Item = HttpResponse, Error = Error> {
      ok((||$code)())
    }
  );

}