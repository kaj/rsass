//! Tests auto-converted from "sass-spec/spec/values/ids.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "// The CSS nav-up property and its siblings allow ID tokens in their values, so\
            \n// Sass parses any tokens that start with a \"#\" followed by an identifier as an\
            \n// ID if it can\'t be parsed as a color.\
            \na {\
            \n  // These IDs are the wrong number of letters to be a hex color.\
            \n  two-letters: #ab;\
            \n  five-letters: #abcde;\
            \n  seven-letters: #abcdefa;\
            \n  nine-letters: #abcdefabc;\
            \n\
            \n  // These IDs contain letters outside the hexadecimal gamut.\
            \n  three-letters-not-hex: #axc;\
            \n  four-letters-not-hex: #axcd;\
            \n  six-letters-not-hex: #abcxde;\
            \n  eight-letters-not-hex: #abcxdefa;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  two-letters: #ab;\
        \n  five-letters: #abcde;\
        \n  seven-letters: #abcdefa;\
        \n  nine-letters: #abcdefabc;\
        \n  three-letters-not-hex: #axc;\
        \n  four-letters-not-hex: #axcd;\
        \n  six-letters-not-hex: #abcxde;\
        \n  eight-letters-not-hex: #abcxdefa;\
        \n}\
        \n"
    );
}
