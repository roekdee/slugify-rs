//! Transliteration of common accented Latin characters to their ASCII
//! equivalents.
//!
//! This is intentionally focused on the Latin-1 / Latin Extended-A range that
//! covers the overwhelming majority of Western European text, plus a handful of
//! frequently seen symbols. It is *not* a full ICU transliteration engine, but
//! it keeps the crate dependency-free while handling the cases people actually
//! hit when slugifying titles and names.

/// Map a single accented or special character to its ASCII representation.
///
/// Returns `Some(&str)` when a mapping exists (the replacement may be more than
/// one ASCII character, e.g. `æ` -> `ae`), or `None` when the character has no
/// known transliteration and should be handled by the caller.
pub(crate) fn deburr_char(c: char) -> Option<&'static str> {
    let mapped = match c {
        // a
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => "a",
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'Ā' | 'Ă' | 'Ą' => "A",
        // ae
        'æ' => "ae",
        'Æ' => "AE",
        // c
        'ç' | 'ć' | 'č' | 'ĉ' | 'ċ' => "c",
        'Ç' | 'Ć' | 'Č' | 'Ĉ' | 'Ċ' => "C",
        // d
        'ð' | 'ď' | 'đ' => "d",
        'Ð' | 'Ď' | 'Đ' => "D",
        // e
        'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' => "e",
        'È' | 'É' | 'Ê' | 'Ë' | 'Ē' | 'Ĕ' | 'Ė' | 'Ę' | 'Ě' => "E",
        // g
        'ĝ' | 'ğ' | 'ġ' | 'ģ' => "g",
        'Ĝ' | 'Ğ' | 'Ġ' | 'Ģ' => "G",
        // h
        'ĥ' | 'ħ' => "h",
        'Ĥ' | 'Ħ' => "H",
        // i
        'ì' | 'í' | 'î' | 'ï' | 'ĩ' | 'ī' | 'ĭ' | 'į' | 'ı' => "i",
        'Ì' | 'Í' | 'Î' | 'Ï' | 'Ĩ' | 'Ī' | 'Ĭ' | 'Į' | 'İ' => "I",
        // j
        'ĵ' => "j",
        'Ĵ' => "J",
        // k
        'ķ' => "k",
        'Ķ' => "K",
        // l
        'ĺ' | 'ļ' | 'ľ' | 'ł' => "l",
        'Ĺ' | 'Ļ' | 'Ľ' | 'Ł' => "L",
        // n
        'ñ' | 'ń' | 'ņ' | 'ň' => "n",
        'Ñ' | 'Ń' | 'Ņ' | 'Ň' => "N",
        // o
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ŏ' | 'ő' => "o",
        'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' | 'Ō' | 'Ŏ' | 'Ő' => "O",
        // oe
        'œ' => "oe",
        'Œ' => "OE",
        // r
        'ŕ' | 'ŗ' | 'ř' => "r",
        'Ŕ' | 'Ŗ' | 'Ř' => "R",
        // s
        'ś' | 'ŝ' | 'ş' | 'š' | 'ș' => "s",
        'Ś' | 'Ŝ' | 'Ş' | 'Š' | 'Ș' => "S",
        // ss
        'ß' => "ss",
        // t
        'ţ' | 'ť' | 'ŧ' | 'ț' => "t",
        'Ţ' | 'Ť' | 'Ŧ' | 'Ț' => "T",
        // u
        'ù' | 'ú' | 'û' | 'ü' | 'ũ' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' => "u",
        'Ù' | 'Ú' | 'Û' | 'Ü' | 'Ũ' | 'Ū' | 'Ŭ' | 'Ů' | 'Ű' | 'Ų' => "U",
        // w
        'ŵ' => "w",
        'Ŵ' => "W",
        // y
        'ý' | 'ÿ' | 'ŷ' => "y",
        'Ý' | 'Ÿ' | 'Ŷ' => "Y",
        // z
        'ź' | 'ż' | 'ž' => "z",
        'Ź' | 'Ż' | 'Ž' => "Z",
        // Common punctuation that reads as a word/space rather than a separator.
        '&' => "and",
        _ => return None,
    };
    Some(mapped)
}
