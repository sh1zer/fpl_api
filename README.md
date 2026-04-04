# Fantasy Premier League API Wrapper (`fpl_client`)
A lightweight, asynchronous Rust wrapper for the official Fantasy Premier League API. This library provides deserialized structs and a simplified interface to easily interact with FPL data in Rust applications.

## Features
- **Asynchronous**: Built with `async`/`await` for efficient, non-blocking HTTP requests.
- **Strongly Typed**: Provides deserialized Rust structs for FPL data models.
- **Simple Interface**: Easy-to-use client for fetching league standings, player data, and more.

## Disclaimer
> **Note:** The official FPL API is undocumented, meaning the exact data structures and responses can change or have unexpected edge cases. While the structs in this library are documented to the best of my knowledge, deserialization errors may occasionally occur. Be cautious!

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
fpl_client = "0.1.1"
```
or run
```bash
cargo add fpl_client
```

## Usage
Here is a basic example of how to use the client to fetch league standings:

```rust
use fpl_client::client::FplApiClient;
use fpl_client::LeagueStandings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a new API client
    let client: FplApiClient = FplApiClient::new()?;

    // 2. Define the league ID you want to fetch
    let league_id: i32 = 123456; 

    // 3. Fetch the standings with explicit type annotation
    let standings: LeagueStandings = client
        .get_league_standings(league_id)
        .await?;

    println!("Fetched standings successfully! League Name: {}", standings.league.name);

    Ok(())
}
```

## Documentation
Can be found at https://docs.rs/fpl_client/0.1.1/fpl_client/
