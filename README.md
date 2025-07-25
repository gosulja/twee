# twee
A Lua/Luau parser for code analysis and editor integration.

# features
* Serialize program into JSON.
* Parses local variable declarations along with type annotations.
* Allows for optional ';' after statements.
* Serializes with proper key-pair value resolution.

# usage + test
```
cargo test
```

or

```
$ cargo build
$ ./twee --input example/input.lua --output example/tree.json
```
