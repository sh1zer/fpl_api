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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
