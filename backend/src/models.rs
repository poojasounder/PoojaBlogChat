use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name=user)]
pub struct NewRustacean {
    pub name: String,
    pub email: String
}
