pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewTask, Task};



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_task(conn: &mut PgConnection, title: &str, body: &str) -> Task {
  use crate::schema::tasks;

  let new_task = NewTask { title, body };

  diesel::insert_into(tasks::table)
      .values(&new_task)
      .get_result(conn)
      .expect("Error saving new task")
}
