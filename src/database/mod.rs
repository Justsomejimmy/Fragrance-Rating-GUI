pub mod schema;
pub mod fragrance_repository;
pub mod user_repository;
pub mod rating_repository;
pub mod note_repository;

use rusqlite::Connection;
use std::path::PathBuf;

fn database_path() -> PathBuf {
    let mut path = dirs::data_dir().expect("Could not determine app data directory");
    path.push("FragranceVault");
    std::fs::create_dir_all(&path).expect("Failed to create app data directory");
    path.push("fragrance.db");
    path
}

pub fn establish_connection() -> Connection {
    let path = database_path();
    println!("Opening database at: {}", path.display());
    Connection::open(&path).expect("Failed to open database")
}