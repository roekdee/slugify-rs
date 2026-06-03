use slugify_rs::{slugify, slugify_with, Options};

#[test]
fn basic_lowercase_and_hyphenation() {
    assert_eq!(slugify("Hello World"), "hello-world");
}

#[test]
fn transliterates_accented_latin() {
    assert_eq!(slugify("Café Déjà"), "cafe-deja");
    assert_eq!(slugify("Crème Brûlée"), "creme-brulee");
    assert_eq!(slugify("Mötley Crüe"), "motley-crue");
    assert_eq!(slugify("Łódź"), "lodz");
}

#[test]
fn expands_multi_char_ligatures() {
    assert_eq!(slugify("Encyclopædia"), "encyclopaedia");
    assert_eq!(slugify("Œuvre"), "oeuvre");
    assert_eq!(slugify("Straße"), "strasse");
}

#[test]
fn ampersand_becomes_and() {
    assert_eq!(slugify("Rock & Roll"), "rock-and-roll");
}

#[test]
fn collapses_symbols_and_punctuation() {
    assert_eq!(slugify("Hello, World!!!"), "hello-world");
    assert_eq!(slugify("a@#$%^b"), "a-b");
    assert_eq!(slugify("100% pure"), "100-pure");
}

#[test]
fn collapses_multiple_spaces() {
    assert_eq!(slugify("a    b      c"), "a-b-c");
    assert_eq!(slugify("tabs\tand\nnewlines"), "tabs-and-newlines");
}

#[test]
fn trims_leading_and_trailing_junk() {
    assert_eq!(slugify("---trim---me---"), "trim-me");
    assert_eq!(slugify("   spaced out   "), "spaced-out");
    assert_eq!(slugify("!!!bang"), "bang");
    assert_eq!(slugify("bang!!!"), "bang");
}

#[test]
fn handles_empty_and_separator_only_input() {
    assert_eq!(slugify(""), "");
    assert_eq!(slugify("   "), "");
    assert_eq!(slugify("---"), "");
    assert_eq!(slugify("!@#$%"), "");
}

#[test]
fn drops_unmapped_unicode_as_separators() {
    // Emoji and non-Latin scripts have no ASCII mapping, so they act as breaks.
    assert_eq!(slugify("hello 😀 world"), "hello-world");
    assert_eq!(slugify("日本語 text"), "text");
}

#[test]
fn custom_separator() {
    let opts = Options::new().separator('_');
    assert_eq!(slugify_with("Hello World", &opts), "hello_world");

    let dot = Options::new().separator('.');
    assert_eq!(slugify_with("a b c", &dot), "a.b.c");
}

#[test]
fn preserve_case_flag() {
    let opts = Options::new().preserve_case(true);
    assert_eq!(slugify_with("Hello World", &opts), "Hello-World");
    assert_eq!(slugify_with("Café Déjà", &opts), "Cafe-Deja");
    assert_eq!(slugify_with("Straße", &opts), "Strasse");
}

#[test]
fn max_length_never_cuts_mid_word() {
    let opts = Options::new().max_length(12);
    assert_eq!(slugify_with("the quick brown fox", &opts), "the-quick");
}

#[test]
fn max_length_keeps_whole_slug_when_it_fits() {
    let opts = Options::new().max_length(100);
    assert_eq!(slugify_with("short and sweet", &opts), "short-and-sweet");
}

#[test]
fn max_length_exact_boundary() {
    // "hello-world" is 11 bytes; a cap of exactly 11 keeps it whole.
    let opts = Options::new().max_length(11);
    assert_eq!(slugify_with("hello world", &opts), "hello-world");

    // A cap of 10 forces a cut back to "hello".
    let tighter = Options::new().max_length(10);
    assert_eq!(slugify_with("hello world", &tighter), "hello");
}

#[test]
fn max_length_first_word_longer_than_limit() {
    // The first word exceeds the cap, so we keep it rather than return nothing.
    let opts = Options::new().max_length(3);
    assert_eq!(
        slugify_with("internationalization wins", &opts),
        "internationalization"
    );
}

#[test]
fn combined_options() {
    let opts = Options::new()
        .separator('_')
        .preserve_case(true)
        .max_length(16);
    // "The_Quick_Brown_Fox" is 19 bytes. slug[..16] is "The_Quick_Brown_",
    // and the last '_' boundary within budget cuts back to "The_Quick_Brown".
    assert_eq!(
        slugify_with("The Quick Brown Fox", &opts),
        "The_Quick_Brown"
    );
}
