//! A Rust wrapper for the official Fantasy Premier League (FPL) API.
//!
//! Provides typed structs for all API responses and an async client for fetching data.
//!
//! # Terminology
//! The FPL API uses some non-obvious naming conventions:
//! - **entry** / **manager** — an FPL player (the person managing a squad)
//! - **element** — a football player
//! - **event** — a gameweek
//!
//! All structs are documented to the best of my knowledge, which means there could be mistakes

pub mod client;
pub mod models;

pub use models::bootstrap_static::BootstrapStatic;
pub use models::fixture::Fixture;
pub use models::league::LeagueStandings;
pub use models::live::LiveEvent;
pub use models::manager::Manager;
pub use models::manager_history::ManagerHistory;
pub use models::picks::ManagerPicks;
pub use models::status::EventStatus;
