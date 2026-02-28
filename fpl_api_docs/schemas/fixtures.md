# Fixture Schema

Endpoint: `fixtures/`

```json
{
  "code": Integer,
  "event": Integer | null,
  "finished": Boolean,
  "finished_provisional": Boolean,
  "id": Integer,
  "kickoff_time": String (ISO datetime) | null,
  "minutes": Integer,
  "provisional_start_time": Boolean,
  "started": Boolean,
  "team_a": Integer,
  "team_a_score": Integer | null,
  "team_h": Integer,
  "team_h_score": Integer | null,
  "stats": [FixtureStat],
  "team_h_difficulty": Integer,
  "team_a_difficulty": Integer,
  "pulse_id": Integer
}
```

### FixtureStat Schema
```json
{
  "identifier": String,
  "a": [StatEntry],
  "h": [StatEntry]
}
```

### StatEntry Schema
```json
{
  "value": Integer,
  "element": Integer (player_id)
}
```
