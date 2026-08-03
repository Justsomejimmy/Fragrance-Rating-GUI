# Fragrance Vault

A native desktop application for managing fragrance collections with custom ratings, seasonal recommendations, searchable notes, and local data storage.

## Overview

Fragrance Vault is a personal fragrance catalog and rating application designed to organize, track, and compare perfumes. Users can maintain a collection, rate fragrances individually, record personal notes, and discover their highest-rated fragrances based on season, scent profile, and personal preferences.

The application is built as a fully local desktop application, allowing all fragrance data and images to remain stored on the user's machine.

## Features

### Collection Management
- Add fragrances to a personal collection
- Upload and store fragrance bottle images
- View fragrance details including:
  - Brand
  - Name
  - Price and date of price
  - Concentration
  - Fragrance notes
  - Recommended seasons
  - Personal reviews

### Rating System
- Individual ratings from multiple users
- 1-10 rating scale
- Compare ratings between users
- Calculate average fragrance ratings

Rating categories include:
- Overall scent
- Longevity
- Projection
- Versatility
- Value

### Organization & Search
- Search fragrances by name, brand, or notes
- Sort collection by:
  - Rating
  - Season
  - Fragrance notes
  - Alphabetical order
  - Price

### Seasonal Recommendations
- View top-rated fragrances based on:
  - Spring
  - Summer
  - Fall
  - Winter

### Wishlist
- Save fragrances to try or purchase later
- Track notes and information about desired fragrances

## Technology Stack

### Application
- Rust
- Tauri
- Slint

### Database
- SQLite

### Development Tools
- Cargo
- Git

## Architecture

```
Fragrance Vault
│
├── User Interface
│   └── Slint UI
│
├── Application Logic
│   └── Rust
│
├── Database Layer
│   └── SQLite
│
└── Local Storage
    ├── Fragrance Data
    └── Bottle Images
```

## Database Design

The application uses a relational database structure to support multiple users, ratings, fragrance notes, and seasonal filtering.

Main entities:

- Fragrances
- Users
- Ratings
- Personal Notes
- Seasons
- Fragrance Notes
- Wishlist Items

The database design supports:
- Multiple season associations per fragrance
- Multiple user ratings
- Searching by fragrance notes
- Future expansion into analytics and recommendation features

## Project Status

Currently in development

Current MVP goals:
- [ ] Create desktop application interface
- [ ] Implement fragrance collection management
- [ ] Add SQLite database integration
- [ ] Add rating system
- [ ] Add search and filtering
- [ ] Add seasonal rankings

## Future Features

Potential improvements:

- Fragrance statistics dashboard
- Collection value tracking
- Wear history tracking
- Bottle usage tracking
- Fragrance recommendation system
- Data export/import
- Backup and restore functionality
- Cloud synchronization

## Installation

### Requirements

- Rust
- Cargo
- Tauri dependencies

### Clone Repository

```bash
git clone <repository-url>

cd fragrance-vault
```

### Run Development Build

```bash
cargo tauri dev
```

## Motivation

This project was created to explore native desktop application development while building a practical tool for organizing and analyzing a personal fragrance collection.

It provides hands-on experience with:
- Rust application development
- Desktop GUI design
- Database architecture
- Local data management
- Software project organization

## License

This project is for personal and educational use.