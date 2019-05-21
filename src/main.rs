#[macro_use] extern crate diesel;
use std::{env, io};
use actix_web::{
  App, HttpRequest, HttpServer,
  guard, web, middleware::{Logger, cors::{Cors}},
};
//custom middleware
// mod middleware;
// use middleware::{Hello,};
// env and util
mod util;
use util::{
  env_json, env_json::{Env},
  db_pool, db_pool::{Pool},
  json_funcs::{
    {json_res, json_msg, json_open},
  },
};

//routes
mod routes;
use routes::{
  redirect, get_test, post_test,
  mock_get, mock_set,
  db_get, db_set
};

fn main() -> io::Result<()> {
  env::set_var("RUST_LOG", "actix_web=debug");
  env_logger::init();
  let sys = actix_rt::System::new("actix-json");
  //load the env vars in env.json
  let env:Env = env_json::init();
  // variables that need to be owned by enclosure
  let pool = db_pool::init(env);
  // build the server - move forces closure to own variables from env (above) e.g. pool
  HttpServer::new(move || {
    App::new()
      .data(pool.clone())
      .wrap(Cors::new()) 
      .wrap(Logger::default())
      //.wrap(Hello)
      // get json tests
      .service(web::resource("/mock_get/{filename}").route(web::get().to_async(mock_get)))
      .service(web::resource("/get_test").route(web::get().to_async(get_test)))
      .service(web::resource("/db_get").route(web::get().to_async(db_get)))
      // post json tests
      .service(web::resource("/mock_set/{filename}").route(web::post().to_async(mock_set)))
      .service(web::resource("/post_test").route(web::post().to_async(post_test)))
      .service(web::resource("/db_set").route(web::post().to_async(db_set)))
      // redirect
      .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
        redirect(req, "get_test")
      })))
      // error endpoint
      .service(web::resource("/error").to(|| {json_msg(500, false, "Server Error")}))
      // default 404 and forbidden
      .default_service(
        web::resource("")
          .route(web::get().to(|| {json_msg(404, false, "Not Found")}))
          .route(web::route().guard(guard::Not(guard::Get())).to(|| {json_msg(403, false, "Forbidden")})),
      )
  })
  .bind("127.0.0.1:8080")?
  .start();

  println!("Starting http server: 127.0.0.1:8080");
  sys.run()
}