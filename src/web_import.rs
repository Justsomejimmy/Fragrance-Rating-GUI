use scraper::{Html, Selector};
use std::io::Read;

pub struct ScrapedFragrance {
    pub brand: String,
    pub name: String,
    pub notes: String,
    pub price: String,
    pub image_path: String,
}

fn meta_content(document: &Html, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(selector_str).ok()?;
    document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .map(|s| s.to_string())
}

fn extract_json_field(json: &serde_json::Value, field: &str) -> Option<String> {
    if let Some(arr) = json.as_array() {
        for item in arr {
            if let Some(v) = extract_json_field(item, field) {
                return Some(v);
            }
        }
        return None;
    }

    if let Some(graph) = json.get("@graph") {
        if let Some(v) = extract_json_field(graph, field) {
            return Some(v);
        }
    }

    match field {
        "name" => json.get("name").and_then(|v| v.as_str()).map(String::from),
        "brand" => json.get("brand").and_then(|b| {
            b.as_str()
                .map(String::from)
                .or_else(|| b.get("name").and_then(|n| n.as_str()).map(String::from))
        }),
        "description" => json.get("description").and_then(|v| v.as_str()).map(String::from),
        "image" => json.get("image").and_then(|v| {
            v.as_str().map(String::from).or_else(|| {
                v.as_array()
                    .and_then(|arr| arr.first())
                    .and_then(|first| first.as_str())
                    .map(String::from)
            })
        }),
        "price" => json.get("offers").and_then(|offers| {
            let offer = if offers.is_array() {
                offers.as_array().and_then(|a| a.first())
            } else {
                Some(offers)
            };
            offer.and_then(|o| o.get("price")).and_then(|p| {
                p.as_str()
                    .map(String::from)
                    .or_else(|| p.as_f64().map(|f| format!("{:.2}", f)))
            })
        }),
        _ => None,
    }
}

fn json_ld_field(document: &Html, field: &str) -> Option<String> {
    let selector = Selector::parse(r#"script[type="application/ld+json"]"#).ok()?;
    for el in document.select(&selector) {
        let text = el.text().collect::<String>();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(value) = extract_json_field(&json, field) {
                return Some(value);
            }
        }
    }
    None
}

fn split_brand_name(title: &str) -> (String, String) {
    for sep in [" - ", " – ", " | ", ": "] {
        if let Some((first, rest)) = title.split_once(sep) {
            return (first.trim().to_string(), rest.trim().to_string());
        }
    }
    (String::new(), title.trim().to_string())
}

fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.trim().to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{}…", truncated.trim())
    }
}

fn download_image(url: &str) -> Option<String> {
    let response = ureq::get(url)
        .set("User-Agent", "Mozilla/5.0 (compatible; FragranceVault/1.0)")
        .timeout(std::time::Duration::from_secs(15))
        .call()
        .ok()?;

    let mut bytes = Vec::new();
    response.into_reader().read_to_end(&mut bytes).ok()?;

    let extension = if url.contains(".png") {
        "png"
    } else if url.contains(".webp") {
        "webp"
    } else {
        "jpg"
    };

    let dir = dirs::data_dir()?.join("FragranceVault").join("imported_images");
    std::fs::create_dir_all(&dir).ok()?;

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_nanos();
    let path = dir.join(format!("import_{}.{}", nanos, extension));

    std::fs::write(&path, &bytes).ok()?;

    Some(path.to_string_lossy().to_string())
}

pub fn fetch_fragrance_info(url: &str) -> Result<ScrapedFragrance, String> {
    let response = ureq::get(url)
        .set("User-Agent", "Mozilla/5.0 (compatible; FragranceVault/1.0)")
        .timeout(std::time::Duration::from_secs(15))
        .call()
        .map_err(|e| format!("Failed to fetch page: {}", e))?;

    let mut body = String::new();
    response
        .into_reader()
        .read_to_string(&mut body)
        .map_err(|e| format!("Failed to read page: {}", e))?;

    let document = Html::parse_document(&body);

    let title = json_ld_field(&document, "name")
        .or_else(|| meta_content(&document, r#"meta[property="og:title"]"#))
        .unwrap_or_default();

    let brand = json_ld_field(&document, "brand")
        .or_else(|| meta_content(&document, r#"meta[property="og:site_name"]"#))
        .unwrap_or_default();

    let description = json_ld_field(&document, "description")
        .or_else(|| meta_content(&document, r#"meta[property="og:description"]"#))
        .or_else(|| meta_content(&document, r#"meta[name="description"]"#))
        .unwrap_or_default();

    let price = json_ld_field(&document, "price")
        .or_else(|| meta_content(&document, r#"meta[property="product:price:amount"]"#))
        .map(|p| if p.starts_with('$') { p } else { format!("${}", p) })
        .unwrap_or_default();

    let image_url = json_ld_field(&document, "image")
        .or_else(|| meta_content(&document, r#"meta[property="og:image"]"#));

    let image_path = match image_url {
        Some(img_url) => download_image(&img_url).unwrap_or_default(),
        None => String::new(),
    };

    let (brand, name) = if brand.is_empty() || brand == title {
        split_brand_name(&title)
    } else {
        (brand, title)
    };

    if brand.is_empty() && name.is_empty() {
        return Err("Couldn't find product information on that page.".to_string());
    }

    Ok(ScrapedFragrance {
        brand,
        name,
        notes: truncate(&description, 400),
        price,
        image_path,
    })
}