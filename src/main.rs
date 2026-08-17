mod database;
mod models;
slint::include_modules!();
use std::rc::Rc;
use slint::{Image, ModelRc, VecModel};

fn join_seasons(spring: bool, summer: bool, fall: bool, winter: bool) -> String {
    let mut parts = Vec::new();
    if spring { parts.push("Spring"); }
    if summer { parts.push("Summer"); }
    if fall { parts.push("Fall"); }
    if winter { parts.push("Winter"); }
    parts.join(",")
}

fn to_fragrance_data(f: &models::fragrance::Fragrance) -> FragranceData {
    let image = if f.image_path.is_empty() {
        Image::default()
    } else {
        match Image::load_from_path(std::path::Path::new(&f.image_path)) {
            Ok(image) => image,
            Err(error) => {
                println!("FAILED to load image for {}: {}", f.name, error);
                Image::default()
            }
        }
    };

    FragranceData {
        id: f.id as i32,
        brand: f.brand.clone().into(),
        name: f.name.clone().into(),
        concentration: f.concentration.clone().into(),
        projection: f.projection.clone().into(),
        longevity: f.longevity.clone().into(),
        price: f.price.clone().into(),
        purchase_date: f.purchase_date.clone().into(),
        rating: f.rating as f32,
        notes: f.notes.clone().into(),
        seasons: f.seasons.clone().into(),
        image,
        my_notes: f.my_notes.clone().into(),
        partner_notes: f.partner_notes.clone().into(),
        image_path: f.image_path.clone().into(),
        image_offset_x: f.image_offset_x as f32,
        image_offset_y: f.image_offset_y as f32,
        image_scale: f.image_scale as f32,
        season_spring: f.seasons.contains("Spring"),
        season_summer: f.seasons.contains("Summer"),
        season_fall: f.seasons.contains("Fall"),
        season_winter: f.seasons.contains("Winter"),
        category: f.category.clone().into(),
    }
}

fn build_dashboard(database: &rusqlite::Connection) -> DashboardData {
    let fragrances = database::fragrance_repository::get_all(database);

    if fragrances.is_empty() {
        return DashboardData {
            total: 0,
            average_rating: 0.0,
            highest_rated: FragranceData::default(),
            lowest_rated: FragranceData::default(),
            most_common_season: "".into(),
            season_top: ModelRc::new(VecModel::from(Vec::<FragranceData>::new())),
            recently_added: ModelRc::new(VecModel::from(Vec::<FragranceData>::new())),
            top_cologne: ModelRc::new(VecModel::from(Vec::<FragranceData>::new())),
            top_perfume: ModelRc::new(VecModel::from(Vec::<FragranceData>::new())),
        };
    }

    let total = fragrances.len();
    let average_rating = (fragrances.iter().map(|f| f.rating).sum::<f64>() / total as f64) as f32;

    let highest = fragrances
        .iter()
        .max_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    let lowest = fragrances
        .iter()
        .min_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    // Tally season counts; ties broken by Spring/Summer/Fall/Winter order.
    let season_order = ["Spring", "Summer", "Fall", "Winter"];
    let mut season_counts = [0usize; 4];

    for f in &fragrances {
        for (i, season) in season_order.iter().enumerate() {
            if f.seasons.contains(season) {
                season_counts[i] += 1;
            }
        }
    }

    let most_common_index = season_counts
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| **count)
        .map(|(i, _)| i)
        .unwrap_or(0);

    let most_common_season = if season_counts[most_common_index] == 0 {
        "".to_string()
    } else {
        season_order[most_common_index].to_string()
    };

    let mut season_matches: Vec<_> = fragrances
        .iter()
        .filter(|f| f.seasons.contains(&most_common_season) && !most_common_season.is_empty())
        .collect();
    season_matches.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
    let season_top: Vec<FragranceData> = season_matches.iter().take(3).map(|f| to_fragrance_data(f)).collect();

    let mut by_recency: Vec<_> = fragrances.iter().collect();
    by_recency.sort_by(|a, b| b.id.cmp(&a.id));
    let recently_added: Vec<FragranceData> = by_recency.iter().take(3).map(|f| to_fragrance_data(f)).collect();

    let mut cologne_matches: Vec<_> = fragrances
        .iter()
        .filter(|f| f.category == "Cologne" || f.category == "Unisex")
        .collect();
    cologne_matches.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
    let top_cologne: Vec<FragranceData> = cologne_matches.iter().take(3).map(|f| to_fragrance_data(f)).collect();

    let mut perfume_matches: Vec<_> = fragrances
        .iter()
        .filter(|f| f.category == "Perfume" || f.category == "Unisex")
        .collect();
    perfume_matches.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
    let top_perfume: Vec<FragranceData> = perfume_matches.iter().take(3).map(|f| to_fragrance_data(f)).collect();

    DashboardData {
        total: total as i32,
        average_rating,
        highest_rated: to_fragrance_data(highest),
        lowest_rated: to_fragrance_data(lowest),
        most_common_season: most_common_season.into(),
        season_top: ModelRc::new(VecModel::from(season_top)),
        recently_added: ModelRc::new(VecModel::from(recently_added)),
        top_cologne: ModelRc::new(VecModel::from(top_cologne)),
        top_perfume: ModelRc::new(VecModel::from(top_perfume)),
    }
}

fn main() {
    let database = database::establish_connection();

    database::schema::create_tables(&database);
    database::schema::seed_database(&database);

    let ui = MainWindow::new().unwrap();
    let database = Rc::new(database);

    let current_sort = Rc::new(std::cell::RefCell::new("Rating: High → Low".to_string()));
    let current_search = Rc::new(std::cell::RefCell::new(String::new()));

    let sort_database = Rc::clone(&database);
    let sort_state = Rc::clone(&current_sort);
    let search_state = Rc::clone(&current_search);
    let sort_ui = ui.as_weak();

    ui.on_refresh_collection(
        move |sort_option| {
            *sort_state.borrow_mut() = sort_option.to_string();
            let search_text = search_state.borrow().clone();

            if let Some(ui) = sort_ui.upgrade() {
                let rows = load_rows(&sort_database, &sort_option, &search_text);
                ui.set_rows(ModelRc::new(VecModel::from(rows)));
            }
        }
    );

    let search_database = Rc::clone(&database);
    let search_sort = Rc::clone(&current_sort);
    let search_state = Rc::clone(&current_search);
    let search_ui = ui.as_weak();

    ui.on_search_collection(
        move |search_text| {
            *search_state.borrow_mut() = search_text.to_string();
            let sort_option = search_sort.borrow().clone();

            if let Some(ui) = search_ui.upgrade() {
                let rows = load_rows(&search_database, &sort_option, &search_text);
                ui.set_rows(ModelRc::new(VecModel::from(rows)));
            }
        }
    );

    let ui_rows = load_rows(&database, "Rating: High → Low", "");
    ui.set_rows(ModelRc::new(VecModel::from(ui_rows)));
    ui.set_dashboard(build_dashboard(&database));

    let save_ui = ui.as_weak();
    let save_database = Rc::clone(&database);
    let save_sort = Rc::clone(&current_sort);
    let save_search = Rc::clone(&current_search);

    ui.on_save_requested(
        move |input| {
            let seasons = join_seasons(
                input.season_spring,
                input.season_summer,
                input.season_fall,
                input.season_winter,
            );

            database::fragrance_repository::update(
                &save_database,
                models::fragrance::UpdateFragrance {
                    id: input.id as i64,
                    brand: &input.brand,
                    name: &input.name,
                    rating: input.rating as f64,
                    concentration: &input.concentration,
                    projection: &input.projection,
                    longevity: &input.longevity,
                    price: &input.price,
                    purchase_date: &input.purchase_date,
                    notes: &input.notes,
                    seasons: &seasons,
                    my_notes: &input.my_notes,
                    partner_notes: &input.partner_notes,
                    image_path: &input.image_path,
                    image_offset_x: input.image_offset_x as f64,
                    image_offset_y: input.image_offset_y as f64,
                    image_scale: input.image_scale as f64,
                    category: &input.category,
                },
            );

            if let Some(ui) = save_ui.upgrade() {
                let search_text = save_search.borrow().clone();
                let sort_option = save_sort.borrow().clone();
                let rows = load_rows(&save_database, &sort_option, &search_text);
                ui.set_rows(ModelRc::new(VecModel::from(rows)));
                ui.set_dashboard(build_dashboard(&save_database));
            }

            println!("Updated fragrance: {}", input.name);
        }
    );

    let image_ui = ui.as_weak();

    ui.on_choose_image_requested(
        move || {
            let mut dialog = rfd::FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "webp"]);

            if let Some(downloads) = dirs::download_dir() {
                dialog = dialog.set_directory(downloads);
            }

            if let Some(path) = dialog.pick_file() {
                println!("Selected image: {}", path.display());

                if let Some(ui) = image_ui.upgrade() {
                    ui.set_selected_image_path(
                        path.to_string_lossy().to_string().into()
                    );
                }
            }
        }
    );

    let add_database = Rc::clone(&database);
    let add_sort = Rc::clone(&current_sort);
    let add_search = Rc::clone(&current_search);
    let add_ui = ui.as_weak();

    ui.on_add_fragrance(
        move |input| {
            let seasons = join_seasons(
                input.season_spring,
                input.season_summer,
                input.season_fall,
                input.season_winter,
            );

            database::fragrance_repository::insert(
                &add_database,
                models::fragrance::NewFragrance {
                    brand: &input.brand,
                    name: &input.name,
                    rating: input.rating as f64,
                    concentration: &input.concentration,
                    projection: &input.projection,
                    longevity: &input.longevity,
                    price: &input.price,
                    purchase_date: &input.purchase_date,
                    notes: &input.notes,
                    seasons: &seasons,
                    image_path: &input.image_path,
                    my_notes: &input.my_notes,
                    partner_notes: &input.partner_notes,
                    image_offset_x: input.image_offset_x as f64,
                    image_offset_y: input.image_offset_y as f64,
                    image_scale: input.image_scale as f64,
                    category: &input.category,
                },
            );

            let sort_option = add_sort.borrow().clone();
            let search_text = add_search.borrow().clone();

            if let Some(ui) = add_ui.upgrade() {
                let rows = load_rows(&add_database, &sort_option, &search_text);
                ui.set_rows(ModelRc::new(VecModel::from(rows)));
                ui.set_dashboard(build_dashboard(&add_database));
            }

            println!("Added fragrance: {}", input.name);
        }
    );

    let delete_ui = ui.as_weak();
    let delete_database = Rc::clone(&database);
    let delete_sort = Rc::clone(&current_sort);
    let delete_search = Rc::clone(&current_search);

    ui.on_delete_requested(
        move |id| {
            database::fragrance_repository::delete(&delete_database, id);

            let sort_option = delete_sort.borrow().clone();
            let search_text = delete_search.borrow().clone();

            if let Some(ui) = delete_ui.upgrade() {
                let rows = load_rows(&delete_database, &sort_option, &search_text);
                ui.set_rows(ModelRc::new(VecModel::from(rows)));
                ui.set_dashboard(build_dashboard(&delete_database));
            }

            println!("Deleted fragrance with ID: {}", id);
        }
    );

    let edit_image_ui = ui.as_weak();

    ui.on_choose_image_requested_edit(
        move || {
            let mut dialog = rfd::FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "webp"]);

            if let Some(downloads) = dirs::download_dir() {
                dialog = dialog.set_directory(downloads);
            }

            if let Some(path) = dialog.pick_file() {
                let path_string = path.to_string_lossy().to_string();
                let picture = slint::Image::load_from_path(&path).unwrap_or_default();

                return ImageSelection {
                    path: path_string.into(),
                    picture,
                };
            }

            ImageSelection {
                path: "".into(),
                picture: Image::default(),
            }
        }
    );

    ui.run().unwrap();
}

fn group_into_rows<T: Clone>(items: &[T], row_size: usize) -> Vec<Vec<T>> {
    items.chunks(row_size).map(|chunk| chunk.to_vec()).collect()
}

fn load_rows(database: &rusqlite::Connection, sort_option: &str, search_text: &str) -> Vec<FragranceRow> {
    let fragrances = database::fragrance_repository::get_all(database);

    let search = search_text.trim().to_lowercase();

    let mut fragrances: Vec<_> = fragrances
        .into_iter()
        .filter(|fragrance| {
            if search.is_empty() {
                return true;
            }
            fragrance.brand.to_lowercase().contains(&search)
                || fragrance.name.to_lowercase().contains(&search)
        })
        .collect();

    match sort_option {
        "Rating: High → Low" => {
            fragrances.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
        }
        "Rating: Low → High" => {
            fragrances.sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap_or(std::cmp::Ordering::Equal));
        }
        "Name: A → Z" => {
            fragrances.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }
        "Name: Z → A" => {
            fragrances.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
        }
        _ => {}
    }

    let ui_fragrances: Vec<FragranceData> = fragrances.iter().map(|f| to_fragrance_data(f)).collect();

    group_into_rows(&ui_fragrances, 3)
        .iter()
        .map(|row| FragranceRow {
            fragrances: ModelRc::new(VecModel::from(row.clone())),
        })
        .collect()
}