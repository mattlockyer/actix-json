use actix_web::{ HttpResponse };
use serde_json::{json, Value as JSON};

pub fn json_res_msg(num:u16, success:bool, message:&str) -> HttpResponse {
  return json_res(num, json!({
    "success": success,
    "message": message
  }));
}

pub fn json_res(num:u16, payload:JSON) -> HttpResponse {
  match num {
    200 => return HttpResponse::Ok().json(payload),
    400 => return HttpResponse::BadRequest().json(payload),
    403 => return HttpResponse::Forbidden().json(payload),
    404 => return HttpResponse::NotFound().json(payload),
    500 => return HttpResponse::InternalServerError().json(payload),
    501 => return HttpResponse::NotImplemented().json(payload),
    _ => return HttpResponse::InternalServerError().json(payload)
  }
}