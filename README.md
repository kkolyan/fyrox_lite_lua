## About
Scripting languages support for Fyrox Engine.

## Structure
It consists from two logically separated things:

_**Fyrox Lite**_ - A Rust library. Provides GC-friendly facade above Fyrox API. Scripting language
bindings are supposed to target this API.

_**Fyrox Lite Lua**_ - Technically, that's a Fyrox game, but completely driven by user-provided Lua scripts. User can bind Lua scripts to nodes in editor the same way as with Rust scripts in vanilla Fyrox. [Lua annotations](https://luals.github.io/wiki/annotations/) are used to define Lua scripts metadata.

## Current State
That's just a demo. It provides a minimal feature set for the [demo shooter game in Lua](https://github.com/kkolyan/fyrox_guards_lua), which is a port of a [demo shooter game in Rust](https://github.com/kkolyan/fyrox_guards).

Exported Fyrox APIs ([annotations available](https://github.com/kkolyan/fyrox_lite_lua/blob/main/fyrox-lite.lua)):
* Node graph.
* RigidBody (3D)
* Ray Casting
* Prefabs & Editor. Everything dynamic in game created in Editor and instantiated via prefabs, except UI, which is coded.

## Plans
1. Reloading of scripts in editor
2. Automatic bindings generation (including docs and annotations) based on the Fyrox Lite for Lua.
3. Support for C# or some another popular statically typed language with GC, using the same approach.
4. API expansion.

# How to run
```sh
git clone https://github.com/kkolyan/fyrox_lite_lua
## assuming there are `scripts/*.lua` in `my_game` folder
cd my_game

## run game
cargo run -p executor-lua --manifest-path ../fyrox_lite_lua/Cargo.toml

## run editor
cargo run -p editor --manifest-path ../fyrox_lite_lua/Cargo.toml
```

## Feedback
Any feedback is extremely appreciated.
Feel free to contact me in Discord `kkolyan_true` ([kkolyan_true](https://discord.com/users/333644000302989314)) or just [create issue](https://github.com/kkolyan/fyrox_lite_lua/issues/new).

## Development
[More information](dev.md)
