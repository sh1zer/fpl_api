# Fantast Premier League API

This project is a simple wrapper for the official Fantasy Premier League api with deserialized structs and a simpler interface.

Not very in-depth, made this mostly for personal use so will probably add more functions as i need

## DESERIALIZATION COULD BE BROKEN I HAVE NOT FOUND A WAY TO VERIFY HOW EXACTLY THE API RETURNS DATA SO THE MODELS COULD BE WRONG


## basic usage:

```rust
async fn test_get_league_standings() {
    // create a client
    let client = FplApiClient::new().expect("Failed to create client");

    // call an endpoint
    let standings = client
        .get_league_standings(LEAGUE_ID)
        .await
        .expect("Failed to fetch league standings");
```
```
```
```
