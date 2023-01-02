use diesel::prelude::*;
use crate::schema::tasks;
use serde::{Serialize};

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_done: bool,
}


#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
