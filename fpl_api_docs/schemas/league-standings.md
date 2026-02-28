# League Standing Schema

Endpoint: `leagues-classic/{league_id}/standings/`

```json
{
  "new_entries": {
    "has_next": Boolean,
    "page": Integer,
    "results": [LeagueEntry]
  },
  "last_updated_data": String (ISO datetime),
  "league": {
    "id": Integer,
    "name": String,
    "created": String (ISO datetime),
    "closed": Boolean,
    "max_entries": Integer | null,
    "league_type": String,
    "scoring": String,
    "admin_entry": Integer | null,
    "start_event": Integer,
    "code_privacy": String,
    "has_cup": Boolean,
    "cup_league": Integer | null,
    "rank": Integer | null
  },
  "standings": {
    "has_next": Boolean,
    "page": Integer,
    "results": [StandingEntry]
  }
}
```

### StandingEntry Schema
```json
{
  "id": Integer,
  "event_total": Integer,
  "player_name": String,
  "rank": Integer,
  "last_rank": Integer,
  "rank_sort": Integer,
  "total": Integer,
  "entry": Integer,
  "entry_name": String
}
```
