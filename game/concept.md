# Description
First person shooter. Player can shoot projectile. Enemies are guards that can shoot character. Characters behavior based on invisible waypoints placed manually around location. Afer shot guard go to the random nearest waypoint and then check for player again. Guards respawned.
- [ ] UI
	- [x] shows amount of hits made to player from guards
	- [x] shows amount of guard kills
- [x] Player
	- [x] Walk
	- [x] Aim
- [x] Bullet
	- [x] kills guards
	- [x] gives frags to player
	- [x] disappear on hit
	- [x] disappear over time
- [ ] Guard
	- [x] walks between waypoints
	- [x] fire at player
	- [ ] check detection sector