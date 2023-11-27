use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, AsChangeset,Serialize, Deserialize)]
pub struct Rustacean {

    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,

    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
