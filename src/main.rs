mod database;
mod models;
slint::include_modules!();
use std::rc::Rc;
use std::collections::HashMap;
use slint::{Image, Model, ModelRc, SharedString, VecModel};

fn join_seasons(spring: bool, summer: bool, fall: bool, winter: bool) -> String {
    let mut parts = Vec::new();
    if spring { parts.push("Spring"); }
    if summer { parts.push("Summer"); }
    if fall { parts.push("Fall"); }
    if winter { parts.push("Winter"); }
    parts.join(",")
}

fn build_ratings_map(database: &rusqlite::Connection) -> HashMap<i64, HashMap<i64, f64>> {
    let all_ratings = database::rating_repository::get_all(database);
    let mut map: HashMap<i64, HashMap<i64, f64>> = HashMap::new();

    for r in all_ratings {
        map.entry(r.fragrance_id).or_insert_with(HashMap::new).insert(r.user_id, r.rating);
    }

    map
}

fn to_fragrance_data(
    f: &models::fragrance::Fragrance,
    users: &[models::user::User],
    ratings_map: &HashMap<i64, HashMap<i64, f64>>,
) -> FragranceData {
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

    let empty_ratings: HashMap<i64, f64> = HashMap::new();
    let fragrance_ratings = ratings_map.get(&f.id).unwrap_or(&empty_ratings);

    let ratings: Vec<UserRating> = users
        .iter()
        .map(|u| UserRating {
            user_id: u.id as i32,
            user_name: u.name.clone().into(),
            rating: *fragrance_ratings.get(&u.id).unwrap_or(&0.0) as f32,
        })
        .collect();

    let average_rating = if fragrance_ratings.is_empty() {
        0.0
    } else {
        (fragrance_ratings.values().sum::<f64>() / fragrance_ratings.len() as f64) as f32
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
        rating: average_rating,
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
        ratings: ModelRc::new(VecModel::from(ratings)),
    }
}

fn group_into_rows<T: Clone>(items: &[T], row_size: usize) -> Vec<Vec<T>> {
    items.chunks(row_size).map(|chunk| chunk.to_vec()).collect()
}

fn load_rows(database: &rusqlite::Connection, sort_option: &str, search_text: &str) -> Vec<FragranceRow> {
    let users = database::user_repository::get_all(database);
    let ratings_map = build_ratings_map(database);
    let fragrances = database::fragrance_repository::get_all(database);

    let mut ui_fragrances: Vec<FragranceData> = fragrances
        .iter()
        .map(|f| to_fragrance_data(f, &users, &ratings_map))
        .collect();

    let search = search_text.trim().to_lowercase();

    ui_fragrances.retain(|f| {
        if search.is_empty() {
            return true;
        }
        f.brand.to_lowercase().contains(&search) || f.name.to_lowercase().contains(&search)
    });

    match sort_option {
        "Rating: High → Low" => {
            ui_fragrances.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
        }
        "Rating: Low → High" => {
            ui_fragrances.sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap_or(std::cmp::Ordering::Equal));
        }
        "Name: A → Z" => {
            ui_fragrances.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }
        "Name: Z → A" => {
            ui_fragrances.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
        }
        _ => {}
    }

    group_into_rows(&ui_fragrances, 3)
        .iter()
        .map(|row| FragranceRow {
            fragrances: ModelRc::new(VecModel::from(row.clone())),
        })
        .collect()
}

fn build_dashboard(database: &rusqlite::Connection) -> DashboardData {
    let users = database::user_repository::get_all(database);
    let ratings_map = build_ratings_map(database);
    let fragrances = database::fragrance_repository::get_all(database);

    let ui_fragrances: Vec<FragranceData> = fragrances
        .iter()
        .map(|f| to_fragrance_data(f, &users, &ratings_map))
        .collect();

    if ui_fragrances.is_empty() {
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

    let total = ui_fragrances.len();
    let average_rating = ui_fragrances.iter().map(|f| f.rating).sum::<f32>() / total as f32;

    let highest = ui_fragrances
        .iter()
        .max_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap()
        .clone();

    let lowest = ui_fragrances
        .iter()
        .min_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap()
        .clone();

    let season_order = ["Spring", "Summer", "Fall", "Winter"];
    let mut season_counts = [0usize; 4];

    for f in &ui_fragrances {
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

    let mut season_matches: Vec<FragranceData> = ui_fragrances
        .iter()
        .filter(|f| !most_common_season.is_empty() && f.seasons.contains(&most_common_season))
        .cloned()
        .collect();
    season_matches.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
    let season_top: Vec<FragranceData> = season_matches.into_iter().take(3).collect();

    let mut by_recency: Vec<FragranceData> = ui_fragrances.clone();
    by_recency.sort_by(|a, b| b.id.cmp(&a.id));
    let recently_added: Vec<FragranceData> = by_recency.into_iter().take(3).collect();

    let mut cologne_matches: Vec<FragranceData> = ui_fragrances
        .iter()
        .filter(|f| f.category == "Cologne" || f.category == "Unisex")
        .cloned()
        .collect();
    cologne_matches.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
    let top_cologne: Vec<FragranceData> = cologne_matches.into_iter().take(3).collect();

    let mut perfume_matches: Vec<FragranceData> = ui_fragrances
        .iter()
        .filter(|f| f.category == "Perfume" || f.category == "Unisex")
        .cloned()
        .collect();
    perfume_matches.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
    let top_perfume: Vec<FragranceData> = perfume_matches.into_iter().take(3).collect();

    DashboardData {
        total: total as i32,
        average_rating,
        highest_rated: highest,
        lowest_rated: lowest,
        most_common_season: most_common_season.into(),
        season_top: ModelRc::new(VecModel::from(season_top)),
        recently_added: ModelRc::new(VecModel::from(recently_added)),
        top_cologne: ModelRc::new(VecModel::from(top_cologne)),
        top_perfume: ModelRc::new(VecModel::from(top_perfume)),
    }
}

fn build_rankings(database: &rusqlite::Connection, mode: &str, option: &str) -> Vec<RankingItem> {
    let users = database::user_repository::get_all(database);
    let ratings_map = build_ratings_map(database);
    let fragrances = database::fragrance_repository::get_all(database);

    let empty_ratings: HashMap<i64, f64> = HashMap::new();

    let mut scored: Vec<(&models::fragrance::Fragrance, f64)> = fragrances
        .iter()
        .filter(|f| match mode {
            "Season" => option.is_empty() || f.seasons.contains(option),
            "Category" => {
                if option == "Unisex" {
                    f.category == "Unisex"
                } else {
                    f.category == option || f.category == "Unisex"
                }
            }
            _ => true,
        })
        .map(|f| {
            let fragrance_ratings = ratings_map.get(&f.id).unwrap_or(&empty_ratings);

            let score = if mode == "User" {
                let user_id = users.iter().find(|u| u.name == option).map(|u| u.id);
                match user_id {
                    Some(uid) => *fragrance_ratings.get(&uid).unwrap_or(&0.0),
                    None => 0.0,
                }
            } else if fragrance_ratings.is_empty() {
                0.0
            } else {
                fragrance_ratings.values().sum::<f64>() / fragrance_ratings.len() as f64
            };

            (f, score)
        })
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    scored
        .into_iter()
        .take(10)
        .map(|(f, score)| {
            let image = if f.image_path.is_empty() {
                Image::default()
            } else {
                Image::load_from_path(std::path::Path::new(&f.image_path)).unwrap_or_default()
            };

            RankingItem {
                id: f.id as i32,
                brand: f.brand.clone().into(),
                name: f.name.clone().into(),
                image,
                image_offset_x: f.image_offset_x as f32,
                image_offset_y: f.image_offset_y as f32,
                image_scale: f.image_scale as f32,
                display_rating: score as f32,
            }
        })
        .collect()
}

fn refresh_users_models(database: &rusqlite::Connection) -> (ModelRc<AppUser>, ModelRc<SharedString>) {
    let users = database::user_repository::get_all(database);

    let app_users: Vec<AppUser> = users
        .iter()
        .map(|u| AppUser { id: u.id as i32, name: u.name.clone().into() })
        .collect();

    let names: Vec<SharedString> = users.iter().map(|u| u.name.clone().into()).collect();

    (
        ModelRc::new(VecModel::from(app_users)),
        ModelRc::new(VecModel::from(names)),
    )
}

fn refresh_all(
    ui: &MainWindow,
    database: &rusqlite::Connection,
    sort_option: &str,
    search_text: &str,
    rank_mode: &str,
    rank_option: &str,
) {
    let rows = load_rows(database, sort_option, search_text);
    ui.set_rows(ModelRc::new(VecModel::from(rows)));
    ui.set_dashboard(build_dashboard(database));

    let (users_model, names_model) = refresh_users_models(database);
    ui.set_users(users_model);
    ui.set_user_names(names_model);

    let rankings = build_rankings(database, rank_mode, rank_option);
    ui.set_ranking_items(ModelRc::new(VecModel::from(rankings)));
}

fn main() {
    let database = database::establish_connection();

    database::schema::create_tables(&database);
    database::schema::seed_database(&database);

    let ui = MainWindow::new().unwrap();
    let database = Rc::new(database);

    let current_sort = Rc::new(std::cell::RefCell::new("Rating: High → Low".to_string()));
    let current_search = Rc::new(std::cell::RefCell::new(String::new()));
    let current_rank_mode = Rc::new(std::cell::RefCell::new("Overall".to_string()));
    let current_rank_option = Rc::new(std::cell::RefCell::new(String::new()));

    refresh_all(&ui, &database, "Rating: High → Low", "", "Overall", "");

    let sort_database = Rc::clone(&database);
    let sort_state = Rc::clone(&current_sort);
    let search_state = Rc::clone(&current_search);
    let sort_rank_mode = Rc::clone(&current_rank_mode);
    let sort_rank_option = Rc::clone(&current_rank_option);
    let sort_ui = ui.as_weak();

    ui.on_refresh_collection(
        move |sort_option| {
            *sort_state.borrow_mut() = sort_option.to_string();
            let search_text = search_state.borrow().clone();
            let rank_mode = sort_rank_mode.borrow().clone();
            let rank_option = sort_rank_option.borrow().clone();

            if let Some(ui) = sort_ui.upgrade() {
                refresh_all(&ui, &sort_database, &sort_option, &search_text, &rank_mode, &rank_option);
            }
        }
    );

    let search_database = Rc::clone(&database);
    let search_sort = Rc::clone(&current_sort);
    let search_state = Rc::clone(&current_search);
    let search_rank_mode = Rc::clone(&current_rank_mode);
    let search_rank_option = Rc::clone(&current_rank_option);
    let search_ui = ui.as_weak();

    ui.on_search_collection(
        move |search_text| {
            *search_state.borrow_mut() = search_text.to_string();
            let sort_option = search_sort.borrow().clone();
            let rank_mode = search_rank_mode.borrow().clone();
            let rank_option = search_rank_option.borrow().clone();

            if let Some(ui) = search_ui.upgrade() {
                refresh_all(&ui, &search_database, &sort_option, &search_text, &rank_mode, &rank_option);
            }
        }
    );

    let save_ui = ui.as_weak();
    let save_database = Rc::clone(&database);
    let save_sort = Rc::clone(&current_sort);
    let save_search = Rc::clone(&current_search);
    let save_rank_mode = Rc::clone(&current_rank_mode);
    let save_rank_option = Rc::clone(&current_rank_option);

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

            for i in 0..input.ratings.row_count() {
                if let Some(user_rating) = input.ratings.row_data(i) {
                    database::rating_repository::set_rating(
                        &save_database,
                        input.id as i64,
                        user_rating.user_id as i64,
                        user_rating.rating as f64,
                    );
                }
            }

            if let Some(ui) = save_ui.upgrade() {
                let search_text = save_search.borrow().clone();
                let sort_option = save_sort.borrow().clone();
                let rank_mode = save_rank_mode.borrow().clone();
                let rank_option = save_rank_option.borrow().clone();
                refresh_all(&ui, &save_database, &sort_option, &search_text, &rank_mode, &rank_option);
            }

            println!("Updated fragrance: {}", input.name);
        }
    );

    let image_ui = ui.as_weak();

    ui.on_choose_image_requested(
        move || {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
                .pick_file()
            {
                println!("Selected image: {}", path.display());

                if let Some(ui) = image_ui.upgrade() {
                    ui.set_selected_image_path(path.to_string_lossy().to_string().into());
                }
            }
        }
    );

    let add_database = Rc::clone(&database);
    let add_sort = Rc::clone(&current_sort);
    let add_search = Rc::clone(&current_search);
    let add_rank_mode = Rc::clone(&current_rank_mode);
    let add_rank_option = Rc::clone(&current_rank_option);
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

            if let Some(ui) = add_ui.upgrade() {
                let sort_option = add_sort.borrow().clone();
                let search_text = add_search.borrow().clone();
                let rank_mode = add_rank_mode.borrow().clone();
                let rank_option = add_rank_option.borrow().clone();
                refresh_all(&ui, &add_database, &sort_option, &search_text, &rank_mode, &rank_option);
            }

            println!("Added fragrance: {}", input.name);
        }
    );

    let delete_ui = ui.as_weak();
    let delete_database = Rc::clone(&database);
    let delete_sort = Rc::clone(&current_sort);
    let delete_search = Rc::clone(&current_search);
    let delete_rank_mode = Rc::clone(&current_rank_mode);
    let delete_rank_option = Rc::clone(&current_rank_option);

    ui.on_delete_requested(
        move |id| {
            database::fragrance_repository::delete(&delete_database, id);
            database::rating_repository::delete_for_fragrance(&delete_database, id as i64);

            if let Some(ui) = delete_ui.upgrade() {
                let sort_option = delete_sort.borrow().clone();
                let search_text = delete_search.borrow().clone();
                let rank_mode = delete_rank_mode.borrow().clone();
                let rank_option = delete_rank_option.borrow().clone();
                refresh_all(&ui, &delete_database, &sort_option, &search_text, &rank_mode, &rank_option);
            }

            println!("Deleted fragrance with ID: {}", id);
        }
    );

    let edit_image_ui = ui.as_weak();

    ui.on_choose_image_requested_edit(
        move || {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
                .pick_file()
            {
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

    let add_user_ui = ui.as_weak();
    let add_user_database = Rc::clone(&database);
    let add_user_sort = Rc::clone(&current_sort);
    let add_user_search = Rc::clone(&current_search);
    let add_user_rank_mode = Rc::clone(&current_rank_mode);
    let add_user_rank_option = Rc::clone(&current_rank_option);

    ui.on_add_user_requested(
        move |name| {
            database::user_repository::insert(&add_user_database, &name);

            if let Some(ui) = add_user_ui.upgrade() {
                let sort_option = add_user_sort.borrow().clone();
                let search_text = add_user_search.borrow().clone();
                let rank_mode = add_user_rank_mode.borrow().clone();
                let rank_option = add_user_rank_option.borrow().clone();
                refresh_all(&ui, &add_user_database, &sort_option, &search_text, &rank_mode, &rank_option);
            }
        }
    );

    let update_user_ui = ui.as_weak();
    let update_user_database = Rc::clone(&database);
    let update_user_sort = Rc::clone(&current_sort);
    let update_user_search = Rc::clone(&current_search);
    let update_user_rank_mode = Rc::clone(&current_rank_mode);
    let update_user_rank_option = Rc::clone(&current_rank_option);

    ui.on_update_user_requested(
        move |id, name| {
            database::user_repository::update(&update_user_database, id as i64, &name);

            if let Some(ui) = update_user_ui.upgrade() {
                let sort_option = update_user_sort.borrow().clone();
                let search_text = update_user_search.borrow().clone();
                let rank_mode = update_user_rank_mode.borrow().clone();
                let rank_option = update_user_rank_option.borrow().clone();
                refresh_all(&ui, &update_user_database, &sort_option, &search_text, &rank_mode, &rank_option);
            }
        }
    );

    let delete_user_ui = ui.as_weak();
    let delete_user_database = Rc::clone(&database);
    let delete_user_sort = Rc::clone(&current_sort);
    let delete_user_search = Rc::clone(&current_search);
    let delete_user_rank_mode = Rc::clone(&current_rank_mode);
    let delete_user_rank_option = Rc::clone(&current_rank_option);

    ui.on_delete_user_requested(
        move |id| {
            database::user_repository::delete(&delete_user_database, id as i64);

            if let Some(ui) = delete_user_ui.upgrade() {
                let sort_option = delete_user_sort.borrow().clone();
                let search_text = delete_user_search.borrow().clone();
                let rank_mode = delete_user_rank_mode.borrow().clone();
                let rank_option = delete_user_rank_option.borrow().clone();
                refresh_all(&ui, &delete_user_database, &sort_option, &search_text, &rank_mode, &rank_option);
            }
        }
    );

    let rank_ui = ui.as_weak();
    let rank_database = Rc::clone(&database);
    let rank_mode_state = Rc::clone(&current_rank_mode);
    let rank_option_state = Rc::clone(&current_rank_option);

    ui.on_rank_requested(
        move |mode, option| {
            *rank_mode_state.borrow_mut() = mode.to_string();
            *rank_option_state.borrow_mut() = option.to_string();

            if let Some(ui) = rank_ui.upgrade() {
                let rankings = build_rankings(&rank_database, &mode, &option);
                ui.set_ranking_items(ModelRc::new(VecModel::from(rankings)));
            }
        }
    );

    ui.run().unwrap();
}