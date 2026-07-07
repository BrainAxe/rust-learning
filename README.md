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
├── count_vowels/       # Practice: count vowels in a string
│   ├── Cargo.toml
│   └── src/main.rs
├── largest_number/     # Practice: find the largest number
│   ├── Cargo.toml
│   └── src/main.rs
├── rectangle/          # Practice: rectangle (structs/methods)
│   ├── Cargo.toml
│   └── src/main.rs
├── reverse_words/      # Practice: reverse words in a string
│   ├── Cargo.toml
│   └── src/main.rs
└── temperature/        # Practice: temperature conversion
    ├── Cargo.toml
    └── src/main.rs
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
