#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod schema;
mod static_files;

fn rocket() -> rocket::Rocket{
    dotenv().ok(); 

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![static_files::all, static_files::index])

}

fn main() {
    rocket().launch();
}