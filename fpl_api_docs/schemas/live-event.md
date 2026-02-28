# Live Event Schema

Endpoint: `event/{event_id}/live/`

```json
{
  "elements": [LivePlayerStat]
}
```

### LivePlayerStat Schema
```json
{
  "id": Integer,
  "stats": {
    "minutes": Integer,
    "goals_scored": Integer,
    "assists": Integer,
    "clean_sheets": Integer,
    "goals_conceded": Integer,
    "own_goals": Integer,
    "penalties_saved": Integer,
    "penalties_missed": Integer,
    "yellow_cards": Integer,
    "red_cards": Integer,
    "saves": Integer,
    "bonus": Integer,
    "bps": Integer,
    "influence": String,
    "creativity": String,
    "threat": String,
    "ict_index": String,
    "starts": Integer,
    "expected_goals": String,
    "expected_assists": String,
    "expected_goal_involvements": String,
    "expected_goals_conceded": String,
    "total_points": Integer,
    "in_dreamteam": Boolean
  },
  "explain": [PointsExplanation],
  "modified": Boolean
}
```

### PointsExplanation Schema
```json
{
  "fixture": Integer,
  "stats": [
    {
      "identifier": String,
      "points": Integer,
      "value": Integer
    }
  ]
}
```
