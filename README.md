# slugify-rs

Turns arbitrary text into URL-safe slugs — `Café Déjà` becomes `cafe-deja`. It transliterates accented Latin characters to ASCII, lowercases, and joins words with hyphens. No dependencies.

![CI](https://github.com/roekdee/slugify-rs/actions/workflows/ci.yml/badge.svg)

## Usage

Add the crate to your `Cargo.toml`, then:

```rust
use slugify_rs::{slugify, slugify_with, Options};

assert_eq!(slugify("Hello, World!"), "hello-world");
assert_eq!(slugify("Crème Brûlée"), "creme-brulee");
assert_eq!(slugify("Rock & Roll"), "rock-and-roll");  // & maps to "and"

// Options is a small builder if you want to change the defaults.
let opts = Options::new()
    .separator('_')
    .preserve_case(true)
    .max_length(16);

assert_eq!(slugify_with("The Quick Brown Fox", &opts), "The_Quick_Brown");
```

Defaults are lowercase, hyphen separator, edges trimmed, and runs of separators collapsed. The `max_length` cap trims at a word boundary rather than mid-word. Anything that isn't ASCII-alphanumeric and has no transliteration (emoji, scripts I don't cover) is treated as a word break, so you get a clean slug instead of garbage.

## Build & test

```bash
cargo test
```

That runs the unit tests, the integration tests in `tests/`, and the doc-tests in the examples above.

## Notes

The accent table is hand-written and covers Latin-1 plus Latin Extended-A — that's most Western European text, including the ligatures people actually hit (`Straße` → `strasse`, `Œuvre` → `oeuvre`). I kept it as a static lookup rather than pulling in a full Unicode normalization crate because the dependency wasn't worth it for the range I care about. The crate is `#![forbid(unsafe_code)]`.

The flip side is that anything outside that table (Cyrillic, Greek, CJK) just drops to separators rather than transliterating. If I needed those, I'd reach for `unicode-normalization` / `deunicode` instead of extending the table by hand.

Rust 2021. CI checks `cargo fmt`, `cargo clippy`, and `cargo test`.

## License

MIT — see [LICENSE](LICENSE).
