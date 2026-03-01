use fpl_api::client::FplApiClient;

#[tokio::test]
async fn test_get_fixtures_event_1() {
    let client = FplApiClient::new().expect("Failed to create client");

    let fixtures = client
        .get_fixtures(1)
        .await
        .expect("Failed to fetch fixtures");

    assert!(!fixtures.is_empty(), "Fixtures list should not be empty");

    // Check first match: id 1, team_a 4
    let first_fixture = &fixtures[0];
    assert_eq!(first_fixture.id, 1, "First fixture ID should be 1");
    assert_eq!(first_fixture.team_a, 4, "First fixture team_a should be 4");
}
