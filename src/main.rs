use std::{env, io};
use actix_web::{
  http::{header, StatusCode},
  middleware::{Logger, cors::{Cors}},
  error, guard, web, App, HttpRequest, HttpResponse, HttpServer,
};

//routes
mod routes;
use routes::{
  not_found, get_test, post_test,
};

//mocks
mod mocks;
use mocks::{
  data_test,
};

//custom middleware
mod middleware;
use middleware::{
  Hello,
};

fn main() -> io::Result<()> {
  env::set_var("RUST_LOG", "actix_web=debug");
  env_logger::init();
  let sys = actix_rt::System::new("actix-json");
  // build the server
  HttpServer::new(|| {
    App::new()
      .wrap(Cors::new()) 
      .wrap(Logger::default())
      .wrap(Hello)
      // get json tests
      .service(web::resource("/get_test").route(web::get().to_async(get_test)))
      .service(web::resource("/data_test").route(web::get().to_async(data_test)))
      // post json tests
      .service(web::resource("/post_test").route(web::post().to_async(post_test)))
      // redirect
      .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
        println!("{:?}", req);
        HttpResponse::Found()
          .header(header::LOCATION, "gettest")
          .finish()
      })))
      // server error endpoint
      .service(web::resource("/error").to(|| {
        error::InternalError::new(
          io::Error::new(io::ErrorKind::Other, "test"),
          StatusCode::INTERNAL_SERVER_ERROR,
        )
      }))
      // default 404 for get and protected for all other
      .default_service(
        web::resource("")
          .route(web::get().to_async(not_found))
          .route(
            web::route()
              .guard(guard::Not(guard::Get()))
              .to(|| HttpResponse::MethodNotAllowed()),
          ),
      )
  })
  .bind("127.0.0.1:8080")?
  .start();

  println!("Starting http server: 127.0.0.1:8080");
  sys.run()
}