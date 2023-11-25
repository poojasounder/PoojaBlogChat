use diesel::{QueryDsl,QueryResult};
use diesel_async::{AsyncPgConnection,RunQueryDsl};

use crate::{schema::rustaceans, models::{Rustacean,NewRustacean}};


pub struct RustaceanRepository;


impl RustaceanRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean>{
        rustaceans::table.find(id).get_result(c).await;
    }
    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Rustacean>>{
        rustaceans::table.limit(limit).get_results(c).await;
    }
    pub async fn create(c: &mut AsyncPgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean>{
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, rustacean:Rustacean) -> QueryResult<Rustacean>{
        
    }
}