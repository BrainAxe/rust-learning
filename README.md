# Rust Learning

A [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) collecting my Rust practice projects. Each folder is a standalone binary crate; they share one `target/` build directory and one `Cargo.lock`.

## Folder structure

```
rust-learning/
├── Cargo.toml          # Workspace file (lists all member projects)
├── Cargo.lock          # Shared lockfile
├── .gitignore          # Ignores /target
├── bank_account/       # Practice: bank account operations
│   ├── Cargo.toml
│   └── src/main.rs
├── counter/            # Practice: counter
│   ├── Cargo.toml
│   └── src/main.rs
└── ...                 # more practice projects
```

## Cargo commands

Run these from the workspace root (`rust-learning/`). Use `-p <project>` to target one member.

| Command | What it does |
|---------|--------------|
| `cargo run -p counter` | Run one project |
| `cargo build` | Build every project |
| `cargo build -p counter` | Build one project |
| `cargo build --release` | Optimized build of everything |
| `cargo check` | Type-check everything without producing binaries |
| `cargo test` | Run all tests |
| `cargo test -p bank_account` | Run tests for one project |
| `cargo clean` | Delete the `target/` build directory |
| `cargo fmt` | Format all code |
| `cargo clippy` | Lint all code |

### Inspecting module structure

Uses [`cargo-modules`](https://crates.io/crates/cargo-modules) (install with `cargo install cargo-modules`).

```bash
cargo modules structure -p auth_service            # print the module/item tree
cargo modules dependencies -p auth_service          # module dependency graph (DOT)
cargo modules dependencies -p auth_service | dot -Tpng -o modules.png   # render to image
```

### Managing dependencies

Add a dependency to a **specific** project with `-p <project>` (run from the workspace root). This edits that project's own `Cargo.toml` and updates the shared `Cargo.lock` — other projects are untouched.

```bash
cargo add rand -p bank_account                    # add the `rand` crate
cargo add serde -p bank_account --features derive # add with a feature enabled
cargo add serde_json@1.0 -p bank_account          # pin a version
cargo add tokio -p bank_account --dev             # dev-dependency (tests/benches only)
cargo remove rand -p bank_account                 # remove a dependency
```

Or `cd bank_account` first and drop the `-p` flag (`cargo add rand`). You can also edit the project's `Cargo.toml` by hand under `[dependencies]` and run `cargo build`.

> Note: tools like `cargo-modules` are installed globally (`cargo install ...`), **not** listed as dependencies — only crates your code actually `use`s belong in `Cargo.toml`.

### Adding a new practice project

```bash
cargo new my_new_exercise
```

Then add it to the `members` list in the root `Cargo.toml`:

```toml
[workspace]
members = [
    # ... existing projects ...
    "my_new_exercise",
]
```
