## About
Scripting languages support for Fyrox Engine.

## Structure
It consists from two logically separated things:

**Fyrox Lite** - A Rust library. Provides GC-friendly facade above Fyrox API. Scripting language
bindings are supposed to target this API.

**Fyrox Lite Lua** - Technically, that's a Fyrox game. Provides Lua bindings for Fyrox Lite. 
In editor mode Lua scripts are attached the same way as Rust scripts attached in vanilla Fyrox.

## Current State
That's just a demo. It provides a minimal feature set for the [demo shooter game](https://github.com/kkolyan/fyrox_guards_lua), which is a Lua port of a [demo shooter game for Fyrox](https://github.com/kkolyan/fyrox_guards).

Exported Fyrox APIs ([annotations available](https://github.com/kkolyan/fyrox_lite_lua/blob/main/fyrox-lite.lua)):
* Node graph.
* RigidBody (3D)
* Ray Casting
* Prefabs & Editor. Everything dynamic in game created in Editor and instantiated via prefabs, except UI, which is coded.

## Plans
1. Reloading of scripts in editor
2. Automatic bindings generation (including docs and annotations) based on the Fyrox Lite for Lua.
3. Support for C# or some another popular statically typed language with GC, using the same approach.
4API expansion.

# How to run
```sh
git clone https://github.com/kkolyan/fyrox_lite_lua
## assuming there are `scripts/*.lua` in `my_game` folder
cd my_game

## run game
cargo run -p executor --manifest-path ../fyrox_lite_lua/Cargo.toml

## run editor
cargo run -p editor --manifest-path ../fyrox_lite_lua/Cargo.toml
```
