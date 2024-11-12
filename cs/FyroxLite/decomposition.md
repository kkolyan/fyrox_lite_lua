### Problems
* I want to keep user code from the entry point class.
* I want to make inaccessible for user the code to bootstrap and engine
* I want to separate editor and player Rust code

### Division
#### FyroxLite
* Contains:
  * The Engine classes like Node, Physics, etc.
  * The abstract classes to derive user scripts from.
  * The attributes to customize scripts serialization.
* Depends on:
  * nothing
#### FyroxLiteNative
* Contains:
  * 