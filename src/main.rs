mod database;
mod models;
slint::include_modules!();

fn main() {
    let database = database::establish_connection();

    database::schema::create_tables(&database);
    
    database::schema::seed_database(&database);

    let fragrances = database::fragrance_repository::get_all(&database);

    println!("{:#?}", fragrances);

    let ui = MainWindow::new().unwrap();

    ui.run().unwrap();
}