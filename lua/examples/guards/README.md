# About
Rudimental Game made with Fyrox Engine and https://github.com/kkolyan/fyrox_lite.

Lua port of https://github.com/kkolyan/fyrox_guards

Explored Fyrox APIs ([annotations available](https://github.com/kkolyan/fyrox_lite_lua/blob/main/fyrox-lite.lua)):
* Node graph.
* RigidBody (3D)
* Ray Casting
* Prefabs & Editor. Everything dynamic in game created in Editor and instantiated via prefabs, except UI, which is coded.

# How to run a game
```sh
git clone https://github.com/kkolyan/fyrox_lite_lua
git clone https://github.com/kkolyan/fyrox_guards_lua
cd fyrox_guards_lua
cargo run -p executor --manifest-path ../fyrox_lite_lua/Cargo.toml
```

# How to play
Use WASD and mouse to shoot enemies and optionally avoid their attacks.

# How to edit scenes
```sh
cargo run -p editor --manifest-path ../fyrox_lite_lua/Cargo.toml
```

# How to edit scripts
Feel free to use any text editor to edit existing files under [scripts](scripts) directory. If you use VSCode, don't forget to download [annotations](https://github.com/kkolyan/fyrox_lite_lua/blob/main/fyrox-lite.lua) for code insight.

![gameplay.png](gameplay.png)