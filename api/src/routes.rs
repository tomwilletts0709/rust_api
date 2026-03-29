use db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{Book, NewBook};
use serde_json::Value;

#[get("/books", format = "application/json")]
fn index (conn: DbConn) -> Json<Value> {
    let books = Book::all(&conn);

    Json(json!({
        "status": 200, 
        "result": books
    }))
}

#[post("/books", format = "application/json", data = "<new_book>")]
fn new(conn: Dbconn, new_book: Json<NewBook>) -> Json<Value> {
    Json(json!({
        "status": Book::insert(new_book.into_inner(), &conn),
        "results": Book::all(&conn).first()
    }))
}

#[get("/books/<id>", format = "application/json")]
fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result = Book::show(id, &conn);
    let status - if result.is_empty() {404} else {200};

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/books/<id>", format = "application/json", data = "<book>")]
fn update(conn: DbConn, id: i32, book: Json<NewBook>) -> Json<Value> {
    let status = Book::update_by_id(id, &conn, book.into_inner());
    let result = Book::show(id, &conn);

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}