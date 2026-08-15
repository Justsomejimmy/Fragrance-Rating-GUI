mod database;
mod models;
slint::include_modules!();
use std::rc::Rc;
use slint::{ModelRc, VecModel};

fn main() {
    let database = database::establish_connection();

    database::schema::create_tables(&database);
    database::schema::seed_database(&database);

    let ui = MainWindow::new().unwrap();

    let ui_rows = load_rows(&database);

    ui.set_rows(
        ModelRc::new(
            VecModel::from(ui_rows)
        )
    );

    // Share the database connection between callbacks
    let database = Rc::new(database);

    let save_ui = ui.as_weak();
    let save_database = Rc::clone(&database);

    ui.on_save_requested(
        move |
            id,
            brand,
            name,
            rating,
            concentration,
            projection,
            longevity,
            price,
            purchase_date,
            notes,
            seasons,
            my_notes,
            partner_notes
        | {
            database::fragrance_repository::update(
                &save_database,
                id as i64,
                &brand,
                &name,
                rating as f64,
                &concentration,
                &projection,
                &longevity,
                &price,
                &purchase_date,
                &notes,
                &seasons,
                &my_notes,
                &partner_notes,
            );

            // Refresh the collection from SQLite
            if let Some(ui) = save_ui.upgrade() {
                let rows = load_rows(&save_database);

                ui.set_rows(
                    ModelRc::new(
                        VecModel::from(rows)
                    )
                );
            }

            println!("Updated fragrance: {}", name);
        }
    );

    let add_database = Rc::clone(&database);
    let add_ui = ui.as_weak();

    ui.on_add_fragrance(
        move |
            brand,
            name,
            rating,
            concentration,
            projection,
            longevity,
            price,
            purchase_date,
            notes,
            seasons,
            my_notes,
            partner_notes
        | {
            database::fragrance_repository::insert(
                &add_database,
                &brand,
                &name,
                rating.parse::<f64>().unwrap_or(0.0),
                &concentration,
                &projection,
                &longevity,
                &price,
                &purchase_date,
                &notes,
                &seasons,
                &my_notes,
                &partner_notes,
            );

            if let Some(ui) = add_ui.upgrade() {
                let rows = load_rows(&add_database);

                ui.set_rows(
                    ModelRc::new(
                        VecModel::from(rows)
                    )
                );
            }

            println!("Added fragrance: {}", name);
        }
    );

    let delete_ui = ui.as_weak();
    let delete_database = Rc::clone(&database);

    ui.on_delete_requested(
        move |id| {
            database::fragrance_repository::delete(
                &delete_database,
                id
            );

            if let Some(ui) = delete_ui.upgrade() {
                let rows = load_rows(&delete_database);

                ui.set_rows(
                    ModelRc::new(
                        VecModel::from(rows)
                    )
                );
            }

            println!("Deleted fragrance with ID: {}", id);
        }
    );

    ui.run().unwrap();
}

fn group_into_rows<T: Clone>(items: &[T], row_size: usize) -> Vec<Vec<T>> {
    items
        .chunks(row_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn load_rows(database: &rusqlite::Connection) -> Vec<FragranceRow> {
    let fragrances = database::fragrance_repository::get_all(database);

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
        })
        .collect::<Vec<_>>();

    group_into_rows(&ui_fragrances, 3)
        .iter()
        .map(|row| {
            FragranceRow {
                fragrances: ModelRc::new(
                    VecModel::from(row.clone())
                ),
            }
        })
        .collect()
}