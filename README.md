# sal-core

## Error

- Error raised in the `Entity` will looks like

```rust
    let dbg = "Entity";
    let err = Error::new(dbg, "Entity raised error");
    assert("Entity | Entity raised error" == err.to_string())
```

- Error raised in the `Entity` and passed up through several classes will be looks like

```rust
    let err = {
        let err = {
            let dbg = "Entity";
            let err = Error::new(dbg, "Entity raised error");
        }
        let dbg = "Wrapper";
        let err = Error::pass(dbg, err);
    }
    let dbg = "RootWrapper";
    let err = Error::new(dbg, err);
    assert_eq!(
        "RootWrapper | \
        \n   └──Wrapper | \
        \n      └──Entity | Entity raised error",
        err.to_string(),
    )
```

## Debug
