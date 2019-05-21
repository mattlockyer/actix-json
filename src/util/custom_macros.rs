#[allow(unused_imports)]
use actix_web::{HttpResponse, Error};
#[allow(unused_imports)]
use futures::{future::{ok}, Future};
/********************************
gen a public function that returns future::ok, but matches actix to_async type req.
params: $name, $code (executed as closure), multiple $arg:$type
********************************/
#[macro_export]
macro_rules! future_ok {
  ($name:ident, $code:expr $(, $arg:ident:$type:ty)*) => (
    pub fn $name($($arg:$type),*) -> impl Future<Item = HttpResponse, Error = Error> {
      ok((||$code)())
    }
  );
}
/********************************
gen a public struct that satisfies the diesel postgres sql constraints
********************************/
#[macro_export]
macro_rules! sql_struct {
  ($name:ident $(, $arg:ident:$type:ty:$sql_type:expr)*) => (
    #[derive(Debug, QueryableByName)]
    #[derive(Serialize, Deserialize)]
    struct $name {
      $(
        #[sql_type=$sql_type]
        $arg:$type,
      )*
    }
  );
}
/********************************
gen a public struct
********************************/
#[macro_export]
macro_rules! struct_test {
  ($name:ident $(, $arg:ident:$type:ty)*) => (
    #[derive(Debug)]
    struct $name {  
      $(
        $arg:$type,
      )*
    }
  );
}
