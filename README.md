# twee
A Lua/Luau parser for code analysis and editor integration.

# features
* Serialize program into JSON.
* Parses local variable declarations along with type annotations.
* Allows for optional ';' after statements.
* Serializes with proper key-pair value resolution.

# example
Test for type annotations for Luau source:
```luau
local name: string = "blinx"
```

Output:
```json
[
    {
        "VariableDecl": {
            "name": "name",
            "value": {
                "String": "blinx"
            },
            "type_annotation": "string"
        }
    }
]
```

# usage + test
```
cargo test
```

or

```
$ cargo build
$ ./twee --input example/input.lua --output example/tree.json
```
