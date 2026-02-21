extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema; 
mod models;

fn main() {
    dotenv::from_path("../.env").ok();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap(); 

    let book = models::NewBook {
        title: String::from("The Power Broker"), 
        author: String::from("Robert Caro"),
        published: true,
    };

    let created = models::Book::create(&conn, book);
    println!("Book inserted successfully with id: {}", created.id);
}