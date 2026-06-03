# slugify-rs

Turn arbitrary Unicode text into clean, URL-safe slugs — accent-aware, dependency-free, and configurable.

![CI](https://github.com/roekdee/slugify-rs/actions/workflows/ci.yml/badge.svg)

## Features

- **Unicode-aware transliteration** — accented Latin characters become ASCII (`Café Déjà` → `cafe-deja`), and ligatures expand correctly (`Straße` → `strasse`, `Œuvre` → `oeuvre`).
- **Sensible defaults** — lowercase, hyphen-separated, edges trimmed, runs of junk collapsed to a single separator.
- **Configurable** via an `Options` builder: custom separator, preserve-case, and a max-length that never cuts a word in half.
- **Graceful degradation** — emoji and unsupported scripts act as word breaks instead of producing garbage.
- **Zero dependencies** — standard library only, `#![forbid(unsafe_code)]`.

## Usage

Add the crate to your `Cargo.toml`, then:

```rust
use slugify_rs::{slugify, slugify_with, Options};

// Defaults: lowercase, hyphen separator, trimmed.
assert_eq!(slugify("Hello, World!"), "hello-world");
assert_eq!(slugify("Crème Brûlée"), "creme-brulee");
assert_eq!(slugify("Rock & Roll"), "rock-and-roll");

// Custom options via a chainable builder.
let opts = Options::new()
    .separator('_')
    .preserve_case(true)
    .max_length(16);

// "The_Quick_Brown_Fox" trimmed to the last word boundary within 16 bytes.
assert_eq!(slugify_with("The Quick Brown Fox", &opts), "The_Quick_Brown");
```

## Build & test

```bash
cargo test
```

This runs the unit tests, the integration tests in `tests/`, and the runnable
examples in the doc comments (doc-tests).

## Tech

- Rust 2021 edition
- Standard library only (no runtime dependencies)
- CI on GitHub Actions: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`

## License

MIT — see [LICENSE](LICENSE).
