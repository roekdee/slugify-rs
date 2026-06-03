//! `slugify-rs` turns arbitrary Unicode text into clean, URL-safe slugs.
//!
//! A slug is the human-readable, lowercase, hyphen-separated fragment you see
//! in URLs like `/blog/my-first-post`. This crate handles the messy real-world
//! input that produces those slugs: accented characters, punctuation, emoji,
//! runs of whitespace, and stray separators at the edges.
//!
//! # Quick start
//!
//! ```
//! use slugify_rs::slugify;
//!
//! assert_eq!(slugify("Café Déjà Vu!"), "cafe-deja-vu");
//! assert_eq!(slugify("  Hello,   World  "), "hello-world");
//! ```
//!
//! # Customising the output
//!
//! Use [`slugify_with`] together with [`Options`] to change the separator,
//! preserve case, or cap the length without cutting a word in half:
//!
//! ```
//! use slugify_rs::{slugify_with, Options};
//!
//! let opts = Options::new().separator('_').preserve_case(true);
//! assert_eq!(slugify_with("Hello World", &opts), "Hello_World");
//!
//! let capped = Options::new().max_length(12);
//! assert_eq!(slugify_with("the quick brown fox", &capped), "the-quick");
//! ```
//!
//! # Design notes
//!
//! * No external dependencies — only the standard library.
//! * Accent transliteration covers Latin-1 and Latin Extended-A, which spans
//!   the common Western European alphabets. See the [`deburr`](crate) logic for
//!   the exact mappings.
//! * Any character that is not ASCII-alphanumeric and has no transliteration is
//!   treated as a separator, so unexpected scripts and symbols degrade
//!   gracefully into clean word breaks instead of garbage.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod deburr;

use deburr::deburr_char;

/// Configuration for [`slugify_with`].
///
/// Build an `Options` value with [`Options::new`] (or [`Options::default`]) and
/// the chainable setters. The defaults match [`slugify`]: a `-` separator,
/// lowercased output, and no length limit.
///
/// ```
/// use slugify_rs::Options;
///
/// let opts = Options::new()
///     .separator('.')
///     .preserve_case(true)
///     .max_length(20);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// Character inserted between words. Defaults to `'-'`.
    separator: char,
    /// Maximum length of the slug in bytes. `None` means unbounded. When set,
    /// the slug is never cut mid-word: it is truncated at the last word
    /// boundary that fits.
    max_length: Option<usize>,
    /// When `true`, the original case of alphanumeric characters is kept.
    /// Defaults to `false` (everything is lowercased).
    preserve_case: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            separator: '-',
            max_length: None,
            preserve_case: false,
        }
    }
}

impl Options {
    /// Create a new `Options` with the default settings (`-` separator,
    /// lowercased, no length cap).
    ///
    /// ```
    /// use slugify_rs::Options;
    ///
    /// let opts = Options::new();
    /// assert_eq!(opts, Options::default());
    /// ```
    pub fn new() -> Self {
        Options::default()
    }

    /// Set the separator character placed between words.
    ///
    /// ```
    /// use slugify_rs::{slugify_with, Options};
    ///
    /// let opts = Options::new().separator('_');
    /// assert_eq!(slugify_with("a b c", &opts), "a_b_c");
    /// ```
    pub fn separator(mut self, separator: char) -> Self {
        self.separator = separator;
        self
    }

    /// Cap the slug to at most `max` bytes, truncating only at a word boundary
    /// so words are never cut in half.
    ///
    /// ```
    /// use slugify_rs::{slugify_with, Options};
    ///
    /// let opts = Options::new().max_length(8);
    /// assert_eq!(slugify_with("hello world", &opts), "hello");
    /// ```
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Preserve the original case of alphanumeric characters instead of
    /// lowercasing them.
    ///
    /// ```
    /// use slugify_rs::{slugify_with, Options};
    ///
    /// let opts = Options::new().preserve_case(true);
    /// assert_eq!(slugify_with("Hello World", &opts), "Hello-World");
    /// ```
    pub fn preserve_case(mut self, preserve: bool) -> Self {
        self.preserve_case = preserve;
        self
    }
}

/// Convert `input` into a URL-safe slug using the default options.
///
/// The output is lowercased, accented Latin characters are transliterated to
/// ASCII, every run of non-alphanumeric characters collapses to a single `-`,
/// and leading/trailing separators are trimmed.
///
/// ```
/// use slugify_rs::slugify;
///
/// assert_eq!(slugify("Hello, World!"), "hello-world");
/// assert_eq!(slugify("Crème Brûlée"), "creme-brulee");
/// assert_eq!(slugify("---trim---me---"), "trim-me");
/// assert_eq!(slugify(""), "");
/// ```
pub fn slugify(input: &str) -> String {
    slugify_with(input, &Options::default())
}

/// Convert `input` into a URL-safe slug using the supplied [`Options`].
///
/// See [`Options`] for the available knobs. This is the workhorse behind
/// [`slugify`].
///
/// ```
/// use slugify_rs::{slugify_with, Options};
///
/// let opts = Options::new().separator('+');
/// assert_eq!(slugify_with("rock & roll", &opts), "rock+and+roll");
/// ```
pub fn slugify_with(input: &str, options: &Options) -> String {
    let sep = options.separator;

    // First pass: build the slug body, collapsing separators as we go. We track
    // `pending_separator` so a run of junk characters only ever emits one
    // separator, and never a leading one.
    let mut out = String::with_capacity(input.len());
    let mut pending_separator = false;

    for ch in input.chars() {
        if let Some(replacement) = normalize_char(ch, options.preserve_case) {
            if pending_separator && !out.is_empty() {
                out.push(sep);
            }
            pending_separator = false;
            out.push_str(&replacement);
        } else {
            // Non-alphanumeric with no transliteration: mark a word break.
            pending_separator = true;
        }
    }

    // A trailing `pending_separator` is intentionally dropped, so the result is
    // already trimmed on both ends.

    match options.max_length {
        Some(max) => truncate_on_boundary(out, sep, max),
        None => out,
    }
}

/// Turn a single character into the slug-ready ASCII text it contributes, or
/// `None` if it should act as a word separator.
fn normalize_char(ch: char, preserve_case: bool) -> Option<String> {
    if ch.is_ascii_alphanumeric() {
        return Some(if preserve_case {
            ch.to_string()
        } else {
            ch.to_ascii_lowercase().to_string()
        });
    }

    if let Some(mapped) = deburr_char(ch) {
        return Some(if preserve_case {
            mapped.to_string()
        } else {
            mapped.to_ascii_lowercase()
        });
    }

    None
}

/// Truncate `slug` to at most `max` bytes without splitting a word.
///
/// If the whole slug already fits, it is returned untouched. Otherwise we cut
/// at the last separator that keeps us within budget. When even the first word
/// is longer than `max`, that single word is returned in full rather than an
/// empty string, because an over-long-but-meaningful slug beats nothing.
fn truncate_on_boundary(slug: String, sep: char, max: usize) -> String {
    if slug.len() <= max {
        return slug;
    }

    match slug[..max].rfind(sep) {
        Some(cut) => slug[..cut].to_string(),
        // No boundary within budget: keep the first whole word.
        None => match slug.find(sep) {
            Some(first_break) => slug[..first_break].to_string(),
            None => slug,
        },
    }
}
