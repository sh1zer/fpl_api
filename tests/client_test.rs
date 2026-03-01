use fpl_api::client::FplApiClient;

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
