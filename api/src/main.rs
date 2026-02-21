#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod schema;
mod static_files;

fn main() {
    dotenv::from_path("../.env").ok();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);

    rocket::build()
        .manage(pool)
        .mount("/", routes![static_files::index, static_files::all])
        .launch();
}

