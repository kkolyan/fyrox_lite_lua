- [Overview](#overview)
- [For users (who make Games)](#for-users-who-make-games)
	- [Vision](#vision)
	- [Current state](#current-state)
	- [How to use Fyrox Lite Lua from code currently](#how-to-use-fyrox-lite-lua-from-code-currently)
	- [Example](#example)
- [For contributors](#for-contributors)
	- [Lite API](#lite-api)
	- [Contract](#contract)
	- [Language Implementations](#language-implementations)
	- [Lua Implementation](#lua-implementation)
- [Feedback](#feedback)

## Overview
Full Scripting languages support for [Fyrox Engine](https://github.com/FyroxEngine/Fyrox). "Full" means that one can make games with Fyrox without seeing a single Rust line of code. The same as Godot, Unity or UE allow to make games with GDSript, C# or Blueprints.

Project ambition is to make Fyrox a "polyglot", so there is an abstraction over the languages, called Lite API, and a number of implementations for different languages. 

For a proof of concept phase `Lua` was chosen, but there is a plan to adopt some popular statically typed language. `C#` and `Kotlin` are main candidates. `$Lang` alias is used further in text instead of refering to a particular language.

## For users (who make Games)

### Vision
When this project is released, this is how games should be made:
1. Download pre-built `Fyrox Lite Toolkit` for a $Lang.
2. Make models/images/sounds using 3rd party tools like Blender.
3. Make scripts using any suitable text editor or IDE.
4. Use the `Fyrox Editor $Lang` (from the Fyrox Lite Toolkit) to create scenes and attach models, images, sound and scripts from the project directory to scene objects.
5. Run a game using a button in Fyrox Editor.
6. Run a game using a `Fyrox Executor $Lang` (from the Fyrox Lite Toolkit), if you like to play without editor.
7. Package game using a `Fyrox Packager $Lang` (from the Fyrox Lite Tooklit).
Project is in deep pre-alpha. There is no downloadable pre-built version for now.

### Current state
1. There is no existing pre-built toolkit yet, so editor and executor should be run from the source code (which is pretty easy actually - see instruction below). The Fyrox Packager doesn't exist at all for now.
2. There is only `Lua` language support currently.
3. There are a lot of temporary limitations, decribed in [known_issues.md](known_issues.md).

### How to use Fyrox Lite Lua from code currently
1. install Rust (https://www.rust-lang.org/tools/install)
2. checkout Fyrox Lite `git clone https://github.com/kkolyan/fyrox_lite_lua` to some directory (let's call it `$FYROX_LITE_HOME`).
3. let's call a directory with your game project files a `$GAME_PROJECT`.
4. open terminal in this directory (`cd $GAME_PROJECT`). That's important - otherwise Fyrox will not be able to find the resources attached to scene objects.
3. Run editor: `cargo run --release -p editor-lua --manifest-path $FYROX_LITE_HOME/Cargo.toml`.
4. Run game without editor: `cargo run --release -p executor-lua --manifest-path $FYROX_LITE_HOME/Cargo.toml`.

### Example
Feel free to chek out a [demo game](lua/examples/guards) that written in Lua to demonstrate the currently Lua-exposed subset of Fyrox API.

## For contributors

### Lite API
Lite API is a Rust library that provides a scripting-language-friendly facade over the Fyrox API. It doesn't bound to a specific language, but it's design assumes that scripting language has GC and some kind of OOP.

This library is supposed to be updated frequently when it's necessary to expose some part of Fyrox API to scripting language. Package [fyrox-lite](fyrox-lite) is the place where most of changes to be done. 

Exposed API should comply with the rules. Following types allowed:
* primitives (limited set of them, for the sake of simplicity)
* `data types` - `#[fyrox_lite]`-annotated structures or enums. they have copy-on-asign semantic. It's supposed that on the scripting language side they are represented in its native data structures. That's not allowed to expose Rust methods of this structures - all necessary methods should be provided by the language specific implementation.
* `engine types` - defined by annotating non-trait `impl`s with `this same #[fyrox_lite]` attribute. Script code can invoke methods (using `ffi` or analogs), but internal structure of this types is completely hidden. Script code can instantiate an engine type only if there is exposed method for this. Handles are clonable and clone operation only clones the handle, not the underlying object. If underlying object has limited lifecycle, then it should provide the methods to deal with it.
* predefined abstract types. that's a family of traits, expected to be implemented by every language provider. they are not intended to be changed frequently. The central type is [UserScript](lua/fyrox-lua/src/user_script_impl.rs).
* Vec, Option, Result

Note that Vector3 and Quaternion for Lua are of an `engine type`, but for some languages (C# for instance) they probably would be a `data type`, because language-native implementation of vector arithmetics could be more efficient than `ffi` to `nalgebra`. That's why nalgebra-backed types are in [fyrox-lite-math](fyrox-lite-math) and [fyrox-lite](fyrox-lite) exposes methods with shallow math structs instead of nalgebra-backed ones.

`#[fyrox_lite]` attrubute is not just a marker - it provides almost complete realtime enforcement of this rules.

### Contract
There is a [metadata model](lite-model/src/lib.rs) that serves as contract between `Lite API` and `Language Implementation`s. There is the [parser](lite-parser) that is responsible to collect metadata using this same `#[lite_api]` attribute.

### Language Implementations
There is no limitation on this. But it's supposed that in consumes the result of Lite API parsing and produces a Rust code with Fyrox `Plugin` implementation that loads scripts metadata, allowing attaching them in inspector, and provides a runtime for a target scripting language.

### Lua Implementation
* `lua/fyrox-lua` - the runtime library, provides [LuaPlugin](lua/fyrox-lua/src/fyrox_plugin.rs) and [LuaScript](lua/fyrox-lua/src/fyrox_script.rs). `mlua` crate used to embed Lua. LuaU interpreter is choosen (mlua allow to switch them easily) just because it was easiest to compile on Windows.
* `lua/editor-lua` / `lua/executor-lua` - desktop instantiations of previously mentioned `LuaPlugin`.
* `lua/luagen-lib` - dynamic part. It uses Lite API metadata to generate both [Lua bindings](lua/fyrox-lua/src/generated) and [Lua annotations](lua/annotations/fyrox-lite.lua) (for autocomplete in VSCode). Currently, it's not integrated in build yet and invoked with `cargo run --bin luagen` ([code](tools/src/bin/luagen.rs)).

## Feedback
Any feedback is extremely appreciated.
Feel free to contact me in Discord `kkolyan_true` ([kkolyan_true](https://discord.com/users/333644000302989314)) or just [create issue](https://github.com/kkolyan/fyrox_lite_lua/issues/new).
