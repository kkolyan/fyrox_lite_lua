## Modules overview

### [fyrox-lite](fyrox-lite)
That's **Lite API** main and mandatory part. The facade over the Fyrox for scripting languages. 
* completely language-agnostic. especially, Rust-agnostic :).
* all exposed APIs are designated with macors from [lite-macro](lite-macro).
* macros also validates exposed APIs on the fly, providing compilation errors for each non-comformity.
* one of the features of this API - it's self contained and doesn't expose any third-party types. one of the notable example - instead of exposing `nalgebra` types, it exposes own shallow Vector3 and Quaternion (simple data structures without any exposed methods).

### [fyrox-lite-math](fyrox-lite-math)
That's the optional part of **Lite API** that exposes facade for some `nalgebra` types, used by Fyrox.
* the embeddable languages like Lua should use bindings to this API.
* the VM-languages like C# doesn't need it, because it's much better to use math library implemented in pure C# than to use P/Invoke for every vector operation.

### [lite-macro](lite-macro) / [lite-macro-lib](lite-macro-lib)
Proc macros to help validate and parse **Lite API**.

### [lite-parser](lite-parser)
Parses **Lite API**s and generates a metadata expressed with data structures from [lite-model](lite-model).

### [fyrox-lite-lua-generator](fyrox-lite-lua-generator)
Generates the Lua language bindings to **Lite API** from the metadata expressed with [lite-model](lite-model). Writes to the subdirectory of [fyrox-lua](fyrox-lua).

### [lite-model](lite-model)
Defines format for metadata, used as contract between **Lite API** and language binding generators.

### [fyrox-lua](fyrox-lua)
It exposes the Lua Plugin, which could be added into any Fyrox executor/editor.
* Technically, that's Fyrox game, completely parameterized by the Lua scripts provided via file system. 
* In editor, Lua Plugin enhances Fyrox Editor with ability attach Lua scripts to nodes.
* Scripts are searched in `${current_dir}/scripts` directory. To be detectable by Fyrox, scripts should be properly annotated, look at the [kkolyan/fyrox_guards_lua](https://github.com/kkolyan/fyrox_guards_lua/tree/master/scripts) sample project for examples.
* besides Node-bound scripts (analog of `fyrox::script::ScriptTrait` implementors in Rust), Lua Plugin has a concept of global scripts, that serves the same role as `fyrox::plugin::Plugin` implementors in Fyrox ([Lua example](https://github.com/kkolyan/fyrox_guards_lua/blob/master/scripts/Game.lua)).
* In executor, Lua Plugin provides runtime for Lua scripts, that are still expected to be in `${current_dir}/scripts` directory. Could be considered as a Lua scripts player.

### [editor-lua](editor)
That's a normal Fyrox editor with Lua Plugin on board. Prebuilt binaries are intended to be downloadable to allow playing or testing Fyrox+Lua games with Fyrox without dealing with any Rust-related things. Or even developing, if you prefer fully-procedural approach.

### [executor-lua](executor-lua)
A normal Fyrox `executor` with Lua Plugin on board. Prebuilt binaries are intended to be downloadable to allow develop games with Fyrox without dealing with any Rust-related things.

### [tools](tools)
Provides entry point for some semi-automated routines.
* [tools/src/bin/genlua.rs](tools/src/bin/genlua.rs) - invokes Lua Bindings generation. Maybe will be wrapped into `build.rs` when project become stable.
