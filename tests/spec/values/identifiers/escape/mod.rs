//! Tests auto-converted from "sass-spec/spec/values/identifiers/escape"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/values/identifiers/escape/normalize"
#[test]
#[ignore] // wrong result
fn normalize() {
    assert_eq!(
        rsass(
            ".normalize {\
            \n  // TODO: remove unnecessary parentheses when we\'re no longer testing against\
            \n  // Ruby Sass.\
            \n  name-start-char: (ax \\61x \\61 x \\061x \\0061x \\00061x);\
            \n  name-start-char-non-hex: (\\xx);\
            \n  name-start-char-non-ascii: (☃x \\☃x \\2603x);\
            \n  name-char-in-middle: (a\\2dx a\\-x);\
            \n  name-char-at-start: (\\2dx \\-x);\
            \n  digit-in-middle: (a\\31x a\\31 x);\
            \n  digit-at-start: (\\31x \\31 x);\
            \n  non-printable: (\\0x \\1x \\2x \\3x \\4x \\5x \\6x \\7x \\8x \\Bx \\Ex \\Fx \\10x \\11x \\12x\
            \n    \\13x \\14x \\15x \\16x \\17x \\18x \\19x \\1Ax \\1Bx \\1Cx \\1Dx \\1Ex \\1Fx \\7Fx);\
            \n  newline: (\\ax \\cx \\dx);\
            \n  tab: (\\\tx \\9x);\
            \n\
            \n  // The beginning of an interpolated identifier should escape name-start chars,\
            \n  // but inner sections should not.\
            \n  name-char-interpolation-beginning: \\-#{foo};\
            \n  name-char-interpolation-middle: #{foo}\\-#{bar};\
            \n  name-char-interpolation-end: #{foo}\\-;\
            \n\
            \n  // Regression test for sass/ruby-sass#94\
            \n  raw-escaped-tab: \\\t;\
            \n}\
            \n\
            \n// Regression test for sass/ruby-sass#96\
            \n@media screen\\9 {\
            \n  x {y: z}\
            \n}\
            \n\
            \nselector\\9 {\
            \n  x: y;\
            \n}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \n.normalize {\
        \n  name-start-char: ax ax ax ax ax ax;\
        \n  name-start-char-non-hex: xx;\
        \n  name-start-char-non-ascii: ☃x ☃x ☃x;\
        \n  name-char-in-middle: a-x a-x;\
        \n  name-char-at-start: \\-x \\-x;\
        \n  digit-in-middle: a1x a1x;\
        \n  digit-at-start: \\31 x \\31 x;\
        \n  non-printable: \\0 x \\1 x \\2 x \\3 x \\4 x \\5 x \\6 x \\7 x \\8 x \\b x \\e x \\f x \\10 x \\11 x \\12 x \\13 x \\14 x \\15 x \\16 x \\17 x \\18 x \\19 x \\1a x \\1b x \\1c x \\1d x \\1e x \\1f x \\7f x;\
        \n  newline: \\a x \\c x \\d x;\
        \n  tab: \\9 x \\9 x;\
        \n  name-char-interpolation-beginning: \\-foo;\
        \n  name-char-interpolation-middle: foo-bar;\
        \n  name-char-interpolation-end: foo-;\
        \n  raw-escaped-tab: \\9 ;\
        \n}\
        \n@media screen\\9  {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \nselector\\9  {\
        \n  x: y;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/values/identifiers/escape/script.hrx"
#[test]
fn script() {
    assert_eq!(
        rsass(
            "// SassScript functions operate on the normalized form of the identifier.\
            \n.script {\
            \n  unescaped-str-length: str-length(ax) str-length(\\61x) str-length(\\00061 x);\
            \n  escaped-str-length: str-length(\\1Ax) str-length(\\0001A x);\
            \n\
            \n  unescaped-slice: str-slice(xaz, 2, 2) str-slice(x\\61z, 2, 2) str-slice(x\\00061 z, 2, 2);\
            \n  escaped-slice: str-slice(x\\1Az, 2, 5) str-slice(x\\0001A z, 2, 5);\
            \n\
            \n  unescaped-quote: quote(ax) quote(\\61x) quote(\\00061 x);\
            \n  escaped-quote: quote(\\1Ax) quote(\\0001A x);\
            \n}\
            \n"
        )
        .unwrap(),
        ".script {\
        \n  unescaped-str-length: 2 2 2;\
        \n  escaped-str-length: 5 5;\
        \n  unescaped-slice: a a a;\
        \n  escaped-slice: \\1a  \\1a ;\
        \n  unescaped-quote: \"ax\" \"ax\" \"ax\";\
        \n  escaped-quote: \"\\\\1a x\" \"\\\\1a x\";\
        \n}\
        \n"
    );
}
