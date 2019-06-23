// FIXME:
// http://www.unicode.org/reports/tr10/#Collation_And_Code_Chart_Order
// http://cldr.unicode.org/

// See FEATURES within Cargo.toml file to select character encoding
#[cfg(feature = "UTF-8")]
const CHAR_ENCODING: Encoding = Encoding::Utf8;
#[cfg(feature = "ISO-8859")]
const CHAR_ENCODING: Encoding = Encoding::Iso8859;
#[cfg(feature = "ASCII-ONLY")]
const CHAR_ENCODING: Encoding = Encoding::AsciiOnly;
#[cfg(not(any(feature="UTF-8", feature="ISO-8859", feature="ASCII-ONLY")))]
const CHAR_ENCODING: Encoding = Encoding::Unspecified;

enum Encoding {
    #[allow(dead_code)]         // For #[cfg...]
    Unspecified,
    #[allow(dead_code)]         // For #[cfg...]
    AsciiOnly,
    #[allow(dead_code)]         // For #[cfg...]
    Iso8859,
    #[allow(dead_code)]         // For #[cfg...]
    Utf8,
}

/// Map encoding of lowercase UTF-8 characters to an index appropriate
/// for accessing elements within an array or vector.  The goal here
/// is *compactness* within that sequence (without Unicode normalization).
///
/// The resulting values within that sequence are unique but otherwise
/// meaningless.
///
/// Maintains uniqueness only within a given natural language's
/// alphabet-- excluding all other languages except for combining
/// modern and historic alphabets of Cyrillic or Greek.
#[inline]
pub fn hash(ch: char) -> Option<usize> {
    let index = match CHAR_ENCODING {
        Encoding::AsciiOnly => {
            // a=97,U+61, z=122,U+7a
            if ch >= '\u{61}' && ch <= '\u{7a}' {
                ch as usize - 0x61
            } else {
                return None
            }
        }
        Encoding::Iso8859 => {
            // Accommodate all of ISO-8859-* (1 through 16).
            // a=97,U+61, z=122,U+7A; 26 lowercase characters
            if ch >= '\u{61}' && ch <= '\u{7a}' { 
                ch as usize - 0x61
            } else if ch >= '\u{A1}' && ch <= '\u{FF}' { // skip NBSP
                // e.g., iso-8859-1 has à=U+00E0, ÿ=U+00FF for lowercase
                // Not the most compact for iso-8859-1 but maintains
                // integrity for Cyrillic in iso-8859-5
                26 + ch as usize - 0xA1
            } else {
                return None
            }
        }
        Encoding::Utf8 => {
            // This is *without* Unich normalization.
            // i.e., UAX #15 *not* implemented due to computational overhead!
            if ch >= '\u{61}' && ch <= '\u{7a}' {
                // ASCII lowercase: a=97,U+61, z=122,U+7a; 26 lowercase chars
                ch as usize - 0x61
            } else if ch >= '\u{00DF}' && ch <= '\u{00FF}' {
                // https://en.wikipedia.org/wiki/Latin-1_Supplement_(Unich_block)
                26 + ch as usize - 0x00DF
            } else if ch >= '\u{0100}' && ch <= '\u{017F}' {
                // https://en.wikipedia.org/wiki/Latin_Extended-A
                // Alternating chars are lowercase
                26 + (ch as usize - 0x0100) / 2
            } else if ch >= '\u{0180}' && ch <= '\u{024F}' {
                // https://en.wikipedia.org/wiki/Latin_Extended-B
                // 97:106 chars are lowercase, thus acceptable to cut in half
                26 + (ch as usize - 0x0180) / 2
            } else if ch >= '\u{03B1}' && ch <= '\u{03C9}' {
                // https://en.wikipedia.org/wiki/Greek_alphabet#Greek_in_Unich
                // Greek and Coptic
                ch as usize - 0x03B1
            } else if ch >= '\u{1F00}' && ch <= '\u{1FF7}' {
                // https://en.wikipedia.org/wiki/Greek_alphabet#Greek_in_Unich
                // Greek Extended: allow for mixing with non-extended
                (0x03C9 - 0x03B1) + ch as usize - 0x1F00
            } else if ch >= '\u{0430}' && ch <= '\u{052D}' {
                // https://en.wikipedia.org/wiki/Cyrillic_(Unich_block)
                // https://en.wikipedia.org/wiki/Cyrillic_script_in_Unich
                // Cyrillic: U+0400–U+04FF, 256 characters
                // Cyrillic Supplement: U+0500–U+052F, 48 characters
                // Cyrillic Extended-C: U+1C80–U+1C8F, 9 characters
                if ch >= '\u{0430}' && ch <= '\u{044F}' {
                    // Basic; lowercase range only: 31 characters
                    ch as usize - 0x0430
                } else if ch >= '\u{0450}' && ch <= '\u{045F}' {
                    // Extensions; lowercase range only: 15 characters
                    31 + ch as usize - 0x0450
                } else if ch >= '\u{0460}' && ch <= '\u{0481}' {
                    // Historic: half of 34 are lowercase: 17 characters
                    31 + 15 + (ch as usize - 0x0460) / 2
                } else if ch >= '\u{048A}' && ch <= '\u{052D}' {
                    // Extended: half of 163 are lowercase: 82 characters
                    31 + 15 + 17 + (ch as usize - 0x048A) / 2
                } else {
                    return None
                }
            } else if ch >= '\u{1C80}' && ch <= '\u{1D78}' {
                // Cyrillic, continued...
                if ch >= '\u{1C80}' && ch <= '\u{1C88}' {
                    // Historic letter variants: all 9 characters
                    31 + 15 + 17 + 82 + ch as usize - 0x1C80
                } else if ch == '\u{1D2B}' {
                    31 + 15 + 17 + 82 + 9 + ch as usize - 0x1D2B
                } else if ch == '\u{1D78}' {
                    31 + 15 + 17 + 82 + 9 + 1 + ch as usize - 0x1D78
                } else {
                    return None
                }
            } else if ch >= '\u{2DE0}' && ch <= '\u{2DFF}' {
                // https://en.wikipedia.org/wiki/Cyrillic_script_in_Unich
                // #Old_Church_Slavonic_combining_letters
                // Cyrillic Extended-A: U+2DE0–U+2DFF, 32 characters
                // all 32 characters
                ch as usize - 0x2DE0
            } else if ch >= '\u{A640}' && ch <= '\u{A69F}' {
                // Cyrillic Extended-B: within U+A640–U+A69F, 96 characters
                if ch >= '\u{A640}' && ch <= '\u{A66D}' {
                    // Cyrillic Extended-B: within U+A640–U+A69F, 96 characters
                    // https://en.wikipedia.org/wiki/Cyrillic_script_in_Unich
                    // #Letters_for_Old_Cyrillic
                    // half of 46 are lowercase: 23 characters
                    (ch as usize - 0xA640) / 2
                } else if ch >= '\u{A674}' && ch <= '\u{A69F}' {
                    // Continuing:
                    // https://en.wikipedia.org/wiki/Cyrillic_script_in_Unich
                    // #Old_Church_Slavonic_combining_letters
                    // Cyrillic Extended-B: within U+A640–U+A69F, 96 characters
                    // all 44 characters
                    23 + ch as usize - 0xA674
                } else if ch >= '\u{A680}' && ch <= '\u{A697}' {
                    // Letters for Old Abkhasian orthography
                    // all 23 characters
                    23 + 44 + ch as usize - 0xA680
                } else if ch == '\u{A699}' {
                    23 + 44 + 23 + ch as usize - 0xA699
                } else if ch == '\u{A69B}' {
                    23 + 44 + 23 + 1 - ch as usize - 0xA69B
                } else {
                    return None
                }
            } else if ch >= '\u{05D0}' && ch <= '\u{05F2}' {
                // https://en.wikipedia.org/wiki/Unich_and_HTML_for_the_Hebrew_alphabet
                // include ligatures for Yiddish for 34 characters
                ch as usize - 0x05D0
            } else if ch >= '\u{FB1D}' && ch <= '\u{FB4F}' {
                // Alphabetic Presentation Forms for Hebrew
                34 + ch as usize - 0xFB1D
            } else {
                return None
            }
        }
        Encoding::Unspecified => return None
    };
    Some(index)
}
