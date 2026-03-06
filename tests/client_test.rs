use fpl_client::client::FplApiClient;

const MANAGER_ID: i32 = 5593884;
const EVENT_ID: i32 = 1;
const ELEMENT_ID: i32 = 1;
const LEAGUE_ID: i32 = 314;

#[tokio::test]
async fn test_get_fixtures_event_1() {
    let client = FplApiClient::new().expect("Failed to create client");

    let fixtures = client
        .get_fixtures(Some(1), None)
        .await
        .expect("Failed to fetch fixtures");

    assert!(!fixtures.is_empty(), "Fixtures list should not be empty");

    // Check first match: id 1, team_a 4
    let first_fixture = &fixtures[0];
    assert_eq!(first_fixture.id, 1, "First fixture ID should be 1");
    assert_eq!(first_fixture.team_a, 4, "First fixture team_a should be 4");
}

#[tokio::test]
async fn test_get_fixtures_event_1_team_1() {
    let client = FplApiClient::new().expect("Failed to create client");

    let fixtures = client
        .get_fixtures(Some(1), Some(1))
        .await
        .expect("Failed to fetch fixtures");

    assert!(!fixtures.is_empty(), "Fixtures list should not be empty");

    let first_fixture = &fixtures[0];
    assert_eq!(first_fixture.id, 9, "First fixture ID should be 9");
    assert_eq!(first_fixture.team_a, 1, "First fixture team_a should be 1");
}

#[tokio::test]
async fn test_get_bootstrap() {
    let client = FplApiClient::new().expect("Failed to create client");

    let bootstrap = client
        .get_bootstrap()
        .await
        .expect("Failed to fetch bootstrap");

    assert!(!bootstrap.elements.is_empty());
    assert!(!bootstrap.teams.is_empty());
    assert!(!bootstrap.events.is_empty());
}

#[tokio::test]
async fn test_get_events() {
    let client = FplApiClient::new().expect("Failed to create client");

    let events = client.get_events().await.expect("Failed to fetch events");

    assert!(!events.is_empty());
}

#[tokio::test]
async fn test_get_event_status() {
    let client = FplApiClient::new().expect("Failed to create client");

    let status = client
        .get_event_status()
        .await
        .expect("Failed to fetch event status");

    assert!(!status.status.is_empty());
}

#[tokio::test]
async fn test_get_live() {
    let client = FplApiClient::new().expect("Failed to create client");

    let live = client
        .get_live(EVENT_ID)
        .await
        .expect("Failed to fetch live data");

    assert!(!live.elements.is_empty());
}

#[tokio::test]
async fn test_get_manager() {
    let client = FplApiClient::new().expect("Failed to create client");

    let manager = client
        .get_manager(MANAGER_ID)
        .await
        .expect("Failed to fetch manager");

    assert_eq!(manager.id, MANAGER_ID);
}

#[tokio::test]
async fn test_get_manager_history() {
    let client = FplApiClient::new().expect("Failed to create client");

    let history = client
        .get_manager_history(MANAGER_ID)
        .await
        .expect("Failed to fetch manager history");

    assert!(!history.current.is_empty());
}

#[tokio::test]
async fn test_get_manager_picks() {
    let client = FplApiClient::new().expect("Failed to create client");

    let picks = client
        .get_manager_picks(MANAGER_ID, EVENT_ID)
        .await
        .expect("Failed to fetch manager picks");

    assert!(!picks.picks.is_empty());
}

#[tokio::test]
async fn test_get_element_summary() {
    let client = FplApiClient::new().expect("Failed to create client");

    let summary = client
        .get_element_matches(ELEMENT_ID)
        .await
        .expect("Failed to fetch element summary");

    assert!(!summary.history.is_empty());
}

#[tokio::test]
async fn test_get_dream_team() {
    let client = FplApiClient::new().expect("Failed to create client");

    let dream_team = client
        .get_dream_team(EVENT_ID)
        .await
        .expect("Failed to fetch dream team");

    assert!(!dream_team.team.is_empty());
}

#[tokio::test]
async fn test_get_league_standings() {
    let client = FplApiClient::new().expect("Failed to create client");

    let standings = client
        .get_league_standings(LEAGUE_ID)
        .await
        .expect("Failed to fetch league standings");

    assert!(!standings.standings.results.is_empty());
}

#[tokio::test]
async fn test_get_league_standings_pages() {
    let client = FplApiClient::new().expect("Failed to create client");

    let standings = client
        .get_league_standings_pages(LEAGUE_ID, 1, 2)
        .await
        .expect("Failed to fetch league standings pages");

    assert!(!standings.standings.results.is_empty());
}

#[tokio::test]
async fn test_get_set_piece() {
    let client = FplApiClient::new().expect("Failed to create client");

    let set_piece = client
        .get_set_piece()
        .await
        .expect("Failed to fetch set piece notes");

    assert!(!set_piece.teams.is_empty());
}
