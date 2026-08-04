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

    let rows = group_into_rows(&ui_fragrances, 3);

    let ui_rows = rows
        .iter()
        .map(|row| {

            FragranceRow {
                fragrances: ModelRc::new(
                    VecModel::from(row.clone())
                )
            }

        }).collect::<Vec<_>>();

    let ui = MainWindow::new().unwrap();

    ui.set_rows(
        ModelRc::new(
            VecModel::from(ui_rows)
        )
    );

    ui.run().unwrap();
}

fn group_into_rows<T: Clone>(items: &[T], row_size: usize) -> Vec<Vec<T>> {
    items
        .chunks(row_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}