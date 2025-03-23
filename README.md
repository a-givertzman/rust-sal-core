# sal-core

## Error

Represents a nested error sequence

> Contains complete path to entity where it was raised, all classes throgh was forwarded up

- As a string may looks like

```log
Root | Root raised error 
    └──Nested-3 | 
       └──Nested-2 | 
          └──Nested-1 | Nested-1 raised error
```

- Error raised in the `Entity`

```rust
    let dbg = "Entity";
    let err = Error::new(dbg, "Entity raised error");
    assert("Entity | Entity raised error" == err.to_string())
```

- Error raised in the `Entity` and passed up through several classes

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
