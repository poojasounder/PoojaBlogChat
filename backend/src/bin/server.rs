extern crate backend;
use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            backend::rocket_routes::authorization::login,
            backend::rocket_routes::rustaceans::get_rustaceans,
            backend::rocket_routes::rustaceans::view_rustacean,
            backend::rocket_routes::rustaceans::create_rustacean,
            backend::rocket_routes::rustaceans::update_rustacean,
            backend::rocket_routes::rustaceans::delete_rustacean,
        ])
        .attach(backend::rocket_routes::CacheConn::init())
        .attach(backend::rocket_routes::DbConn::init())
        .launch()
        .await;
}