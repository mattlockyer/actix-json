use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
// There are two step in middleware processing.
// 1. Middleware initialization, middleware factory get called with
//    next service in chain as parameter.
// 2. Middleware's call method get called with normal request.
pub struct Hello;
// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Hello
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = HelloMiddleware<S>;
  type Future = FutureResult<Self::Transform, Self::InitError>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(HelloMiddleware { service })
  }
}

pub struct HelloMiddleware<S> {
  service: S,
}

impl<S, B> Service for HelloMiddleware<S>
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

  fn poll_ready(&mut self) -> Poll<(), Self::Error> {
    self.service.poll_ready()
  }

  fn call(&mut self, req: ServiceRequest) -> Self::Future {
    println!("Hi from start. You requested: {}", req.path());

    Box::new(self.service.call(req).and_then(|res| {
      println!("Hi from response");
      Ok(res)
    }))
  }
}