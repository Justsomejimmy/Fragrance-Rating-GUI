mod database;
mod models;
slint::include_modules!();
use slint::{ModelRc, VecModel};

fn main() {
    let database = database::establish_connection();

    database::schema::create_tables(&database);
    
    database::schema::seed_database(&database);

    let fragrances = database::fragrance_repository::get_all(&database);

    println!("{:#?}", fragrances);

    let ui = MainWindow::new().unwrap();

    let fragrances = database::fragrance_repository::get_all(&database);

    let ui_fragrances = fragrances
        .iter()
        .map(|f| {
            FragranceData {
                brand: f.brand.clone().into(),
                name: f.name.clone().into(),
                rating: f.rating,
                notes: f.notes.clone().into(),
                seasons: f.seasons.clone().into(),
            }
        }).collect::<Vec<_>>();


    ui.set_fragrances(
        ModelRc::new(
            VecModel::from(ui_fragrances)
        )
    );

    ui.run().unwrap();
}