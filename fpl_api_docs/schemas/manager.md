# Manager/Entry Schema

Endpoint: `entry/{manager_id}/`

```json
{
  "id": Integer,
  "joined_time": String (ISO datetime),
  "started_event": Integer,
  "favourite_team": Integer | null,
  "player_first_name": String,
  "player_last_name": String,
  "player_region_id": Integer,
  "player_region_name": String,
  "player_region_iso_code_short": String,
  "player_region_iso_code_long": String,
  "summary_overall_points": Integer | null,
  "summary_overall_rank": Integer | null,
  "summary_event_points": Integer | null,
  "summary_event_rank": Integer | null,
  "current_event": Integer | null,
  "leagues": {
    "classic": [LeagueEntry],
    "h2h": [LeagueEntry],
    "cup": CupStatus,
    "cup_matches": [CupMatch]
  },
  "name": String,
  "name_change_blocked": Boolean,
  "kit": String | null,
  "last_deadline_bank": Integer | null,
  "last_deadline_value": Integer | null,
  "last_deadline_total_transfers": Integer | null,
  "entered_events": [Integer],
  "years_active": [Integer],
  "club_badge_src": String | null
}
```
