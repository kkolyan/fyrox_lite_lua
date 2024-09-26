# Entities of Lite framework

## Engine Class
* doesn't expose its internal structure to scripting language. in Lua it's represented with `userdata`, in Java-like runtimes it's represented as "handle" class with only id field.
* can only be instantiated using factory methods provided by engine
* can only have methods. fields can be emulated only for languages with native getter/setter support, like in Lua or C#.
* in Lite API it's defined using a proc macro above the `impl` block of some struct or enum.

## User Class
* in Lite API it's defined using a trait with the proc macro


# Thoughts
* seems like we need to provide Engine Classes for Vector3 and Quaternion on the level of the Lua bindings, because Java-like languages will use their own implementations.
