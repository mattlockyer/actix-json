use futures::{Future, future::{ok}};
use serde_json::{ json, from_value, Value as JSON};
use serde_derive::{Deserialize, Serialize, };
use diesel::{
  sql_types::{Int4, Text,},
  sql_query, query_dsl::RunQueryDsl,
  pg::PgConnection, r2d2::{ self, ConnectionManager },
};
use actix_web::{ 
  web::{ Data, Json, },
  HttpResponse, Error 
};
// this crate
use crate::future_ok;
use super::{json_res, json_msg};
use super::{Env};
/********************************
Create a connection pool, connect to postgres
********************************/
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn init(env: Env) -> Pool {
  let manager = ConnectionManager::<PgConnection>::new(format!(
    "postgresql://{}:{}@localhost", env.postgres.username, env.postgres.password,
  ));
  r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
/********************************
Struct for table test.item, QueryableByName for raw SQL, use sql_type for each field
********************************/
#[derive(Debug, QueryableByName)]
#[derive(Serialize, Deserialize)]
struct Item {
  #[sql_type="Int4"]
  id: i32,
  #[sql_type="Text"]
  data: String,
}
/********************************
Get all rows and columns in table test.item
********************************/
future_ok!(db_get,
  {
    let conn: &PgConnection = &pool.get().unwrap();
    let items:Vec<Item> = sql_query("SELECT id, data FROM test.item").load(conn).unwrap();
    // for item in items {
    //   println!("Found item {}: {}", item.id, item.data);
    // }
    return json_res(200, json!({
      "items": items
    }));
  },
  pool:Data<Pool>
);
/********************************
Insert a new row into table test.item
********************************/
future_ok!(db_set,
  {
    let conn: &PgConnection = &pool.get().unwrap();
    let item:Item = from_value(body.into_inner()).unwrap();
    let result:Vec<Item> = sql_query("INSERT INTO test.item (data) VALUES ($1)")
      .bind::<Text, _>(item.data)
      .load(conn)
      .expect("INSERT ERROR");
    println!("{:?}", result);
    return json_msg(200, true, "Item inserted");
  },
  pool:Data<Pool>, body:Json<JSON>
);
