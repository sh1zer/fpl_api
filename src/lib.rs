pub mod models;

pub use models::bootstrap_static::BootstrapStatic;
pub use models::fixture::Fixture;
pub use models::league::LeagueStandings;
pub use models::live::LiveEvent;
pub use models::manager::Manager;
pub use models::picks::ManagerPicks;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
