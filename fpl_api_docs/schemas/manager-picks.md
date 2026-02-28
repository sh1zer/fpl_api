# Manager Picks Schema

Endpoint: `entry/{manager_id}/event/{event_id}/picks/`

```json
{
  "active_chip": String | null,
  "automatic_subs": [AutomaticSub],
  "entry_history": GameweekHistory,
  "picks": [Pick]
}
```

### Pick Schema
```json
{
  "element": Integer (player_id),
  "position": Integer (1-15),
  "multiplier": Integer,
  "is_captain": Boolean,
  "is_vice_captain": Boolean,
  "element_type": Integer
}
```

### AutomaticSub Schema
```json
{
  "entry": Integer,
  "element_in": Integer,
  "element_out": Integer,
  "event": Integer
}
```
