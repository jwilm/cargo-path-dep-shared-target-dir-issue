cargo-path-dep-shared-target-dir-issue
======================================

There _may_ be an issue with Cargo not rebuilding dependencies in certain
situations.

Projects A and B are simple libraries with a single source file. B's Cargo.toml
depends on A via a relative path. The .cargo/config specifies a shared target
directory.

```
├── .cargo
│   └── config
├── A
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── B
    ├── Cargo.toml
    └── src
        └── lib.rs
```

## Triggering the broken behavior

First, build both projects

```
cd A && cargo test && cd -
cd B && cargo test && cd -
```

Now, change the `trait Foo` in crate A. B implements it for some B type, so it
should get a compile error if the trait is changed. The `foo` function returns a
`u32` to start with. Making it return `f32` instead should cause B compilation
to fail.

Build both projects again. If A is built first, B compilation will succeed
erroneously. However, if B is built first, compilation will fail as expected.
