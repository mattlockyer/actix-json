use std::{env, io};
use actix_web::{
  App, HttpRequest, HttpServer,
  guard, web, middleware::{Logger, cors::{Cors}},
};

//util
mod util;
use util::{json_res_msg};

//routes
mod routes;
use routes::{redirect, get_test, post_test,};

//mocks
mod mocks;
use mocks::{mock_get, mock_set,};

//custom middleware
mod middleware;
use middleware::{Hello,};

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
      .service(web::resource("/mock_get/{filename}").route(web::get().to_async(mock_get)))
      .service(web::resource("/get_test").route(web::get().to_async(get_test)))
      // post json tests
      .service(web::resource("/mock_set/{filename}").route(web::post().to_async(mock_set)))
      .service(web::resource("/post_test").route(web::post().to_async(post_test)))
      // redirect
      .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
        redirect(req, "get_test")
      })))
      // error endpoint
      .service(web::resource("/error").to(|| {json_res_msg(500, false, "Server Error")}))
      // default 404 and forbidden
      .default_service(
        web::resource("")
          .route(web::get().to(|| {json_res_msg(404, false, "Not Found")}))
          .route(web::route().guard(guard::Not(guard::Get())).to(|| {json_res_msg(403, false, "Forbidden")})),
      )
  })
  .bind("127.0.0.1:8080")?
  .start();

  println!("Starting http server: 127.0.0.1:8080");
  sys.run()
}