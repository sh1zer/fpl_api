use crate::LeagueStandings;
use crate::client::FplApiClient;
use crate::models::fixture::Fixture;
use anyhow::Result;
use std::collections::HashMap;

impl FplApiClient {
    /// fetches the first page of a league standings (50 entries)
    pub async fn get_league_standings(&self, league_id: i32) -> Result<LeagueStandings> {
        self.gets(format!("leagues-classic/{}/standings", league_id).as_str())
            .await
    }

    /// fetches any range of pages from league standings, every page is 50 entries
    pub async fn get_league_standings_pages(
        &self,
        league_id: i32,
        pages_start: i32,
        pages_end: i32,
    ) -> Result<LeagueStandings> {
        let mut base: LeagueStandings = self
            .get(
                format!("leagues-classic/{}/standings", league_id).as_str(),
                Some([("page_standings", pages_start)]),
            )
            .await?;

        if !base.standings.has_next {
            return Ok(base);
        }

        for page_id in pages_start + 1..pages_end {
            let page: LeagueStandings = self
                .get(
                    format!("leagues-classic/{}/standings", league_id).as_str(),
                    Some([("page_standings", page_id)]),
                )
                .await?;

            base.standings.results.extend(page.standings.results);

            if !page.standings.has_next {
                break;
            }
        }

        Ok(base)
    }
}
