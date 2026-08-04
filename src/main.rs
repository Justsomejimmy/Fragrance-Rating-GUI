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
                id: f.id,
                brand: f.brand.clone().into(),
                name: f.name.clone().into(),
                concentration: f.concentration.clone().into(),
                projection: f.projection.clone().into(),
                longevity: f.longevity.clone().into(),
                price: f.price.clone().into(),
                purchase_date: f.purchase_date.clone().into(),
                rating: f.rating,
                notes: f.notes.clone().into(),
                seasons: f.seasons.clone().into(),
                image_path: f.image_path.clone().into(),
                my_notes: f.my_notes.clone().into(),
                partner_notes: f.partner_notes.clone().into(),
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