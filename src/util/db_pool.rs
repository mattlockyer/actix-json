use diesel::{
  pg::PgConnection, r2d2::{ self, ConnectionManager },
};
/********************************
Create a connection pool, connect to postgres
********************************/
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn init(username: String, password: String) -> Pool {
  let manager = ConnectionManager::<PgConnection>::new(format!(
    "postgresql://{}:{}@localhost", username, password,
  ));
  r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}