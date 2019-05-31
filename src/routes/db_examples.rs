
use futures::{Future, future::{ok}};
use serde_json::{ json, from_value, Value as JSON};
use serde_derive::{Deserialize, Serialize, };
use actix_web::{ 
  web::{ Data, Json, },
  HttpResponse, Error 
};
use diesel::{
  sql_types::{Int4, Text,},
  sql_query, query_dsl::RunQueryDsl,
  pg::PgConnection, 
};
// this crate
use crate::{Pool, json_res, json_msg, fres, sql_struct};
/********************************
Struct for table test.item, QueryableByName for raw SQL, use sql_type for each field
********************************/
sql_struct!(Item, id:i32:"Int4", data:String:"Text");
/********************************
Get all rows and columns in table test.item
********************************/
pub fn db_get(pool:Data<Pool>) -> fres!() {
  let conn: &PgConnection = &pool.get().unwrap();
  let items:Vec<Item> = sql_query("SELECT id, data FROM test.item").load(conn).unwrap();
  // for item in items {
  //   println!("Found item {}: {}", item.id, item.data);
  // }
  ok(json_res(200, json!({
    "items": items
  })))
}
/********************************
Insert a new row into table test.item
********************************/
pub fn db_set(pool:Data<Pool>, body:Json<JSON>) -> fres!() {
  let conn: &PgConnection = &pool.get().unwrap();
  let item:Item = from_value(body.into_inner()).unwrap();
  let result:Vec<Item> = sql_query("INSERT INTO test.item (data) VALUES ($1)")
    .bind::<Text, _>(item.data)
    .load(conn)
    .expect("INSERT ERROR");
  println!("{:?}", result);
  ok(json_msg(200, true, "Item inserted"))
}
