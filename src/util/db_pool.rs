use diesel::{
  pg::PgConnection, r2d2::{ self, ConnectionManager },
};
use crate::{Env};
/********************************
Create a connection pool, connect to postgres
********************************/
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub fn init(env: Env) -> Pool {
  let manager = ConnectionManager::<PgConnection>::new(format!(
    "postgresql://{}:{}@localhost", env.postgres.username, env.postgres.password,
  ));
  r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}