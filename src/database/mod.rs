pub mod schema;
pub mod fragrance_repository;
pub mod user_repository;
pub mod rating_repository;

use rusqlite::Connection;

pub fn establish_connection() -> Connection {
    Connection::open("fragrance.db").expect("Failed to open databse")
}