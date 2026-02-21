use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::books;

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub published: bool,
}

impl Book {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Book> {
        books::table
            .find(id)
            .load::<Book>(conn)
            .expect("Error loading book")
    }

    pub fn all(conn: &PgConnection) -> Vec<Book> {
        books::table
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect("Error loading books")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, book: NewBook) -> bool {
        use books::dsl::{author as a, published as p, title as t};
        let NewBook { title, author, published } = book;

        diesel::update(books::table.find(id))
            .set((a.eq(author), p.eq(published), t.eq(title)))
            .get_result::<Book>(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Book::show(id, conn).is_empty() {
            return false;
        }

        diesel::delete(books::table.find(id)).execute(conn).is_ok()
    }

    pub fn all_by_author(author: String, conn: &PgConnection) -> Vec<Book> {
        books::table
            .filter(books::author.eq(author))
            .load::<Book>(conn)
            .expect("Error loading books")
    }

    pub fn create(conn: &PgConnection, book: NewBook) -> Book {
        diesel::insert_into(books::table)
            .values(&book)
            .get_result(conn)
            .expect("Error inserting book")
    }
}
