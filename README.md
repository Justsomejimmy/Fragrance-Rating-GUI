# Fragrance Vault

A native desktop application for managing a fragrance collection — with multi-user ratings, seasonal recommendations, searchable notes, and fully local data storage.

## Overview

Fragrance Vault is a personal fragrance catalog and rating application for organizing, tracking, and comparing perfumes. Multiple users can each rate a fragrance individually, and the app surfaces averaged ratings, seasonal top picks, and a top-10 rankings view. All data and images are stored locally — nothing leaves the user's machine.

## Features

**Collection & Wishlist**
- Add fragrances to a collection or a separate wishlist, and move items between the two
- Upload a bottle image with pan/zoom cropping
- Track brand, name, price, purchase date, concentration, projection, longevity, category, and seasons
- Auto-fill fragrance details from a product link (Open Graph / schema.org metadata)

**Ratings**
- Multiple users, each with their own 0–10 rating per fragrance
- Automatically averaged and displayed on the collection and details views
- Manage users (add, rename, remove) from Settings

**Notes**
- Searchable, creatable fragrance-notes system shared across the collection
- Filter the collection by a specific note

**Organization & Discovery**
- Search by brand or name; sort by rating, name, category, price, purchase date, notes, or a specific user's rating
- Dashboard with collection totals, average rating, highest/lowest rated, and top picks by season and category
- Top-10 rankings, filterable by season, category, or user

**Data Validation**
- Required-field and format checks (price, date) on save, with inline error messages

## Technology Stack

- **Application:** Rust, Slint
- **Database:** SQLite (via `rusqlite`)
- **Web Import:** `ureq`, `scraper`, `serde_json`
- **Tooling:** Cargo, Git

## Architecture

```
Fragrance Vault
│
├── UI            → Slint
├── App Logic     → Rust
├── Database      → SQLite
└── Local Storage → Fragrance data & bottle images
```

## Database Design

- **fragrances** — core fragrance data, seasons, category, image, wishlist flag
- **users** — people who rate fragrances
- **ratings** — per-user rating per fragrance
- **note_options** — the shared, searchable catalog of fragrance notes

## Completed Features

- [x] Desktop application interface
- [x] Collection and wishlist management
- [x] SQLite database integration
- [x] Multi-user rating system
- [x] Search, sort, and filter
- [x] Dashboard and seasonal/category rankings
- [x] Searchable, creatable fragrance notes
- [x] Website auto-import
- [x] Field validation
- [x] Adjustable, auto-scaling window sizing

## Planned

- Packaged standalone executable
- CI/CD pipeline

## Installation

**Requirements:** Rust, Cargo

```bash
git clone <repository-url>
cd fragrance-vault
cargo run
```

## License

This project is for personal and educational use.