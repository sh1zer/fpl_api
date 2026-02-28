## base url
https://fantasy.premierleague.com/api/

### general data (info about all teams and all players)
bootstrap-static/

### every fixture of the season
fixtures/
works with parameters 
/?team={team_id}
/?event={event_id}

### specific player data
element-summary/{element_id}/

### data for all gameweeks in the season
events/

### live data for given gameweek
event/{event_id}/live/

### manager information
entry/{manager_id}/

### manager history (specific gw performances mostly)
entry/{manager_id}/history

### league standings
leagues-classic/{league_id}/standings

### managers team for a given gw
entry/{manager_id}/event/{event_id}/picks/

### check if the bonus points have been updated across the api
event-status/

### highest scoring team for the given gw
dream-team/{event_id}/

### set piece takers info
team/set-piece-notes/

### some authenticated endpoints also exist that i cannot be bothered to implement as they dont fit my use case

terminology:
fixture - match
element - football player
event - gameweek
manager - fpl player (like me and you)

