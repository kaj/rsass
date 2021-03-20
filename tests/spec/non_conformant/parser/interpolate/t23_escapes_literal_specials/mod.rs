//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials"

mod t01_inline;

mod t02_variable;

mod t03_inline_double;

mod t04_variable_double;

mod t05_variable_quoted_double;

// From "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/06_escape_interpolation"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        crate::rsass(
            "$input: \\0_\\a_\\A;\
            \n.result {\
            \n  output: \"[\\#{\\0_\\a_\\A}]\";\
            \n  output: \"\\#{\\0_\\a_\\A}\";\
            \n  output: \'\\#{\\0_\\a_\\A}\';\
            \n  output: \"[\'\\#{\\0_\\a_\\A}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \n.result {\
        \n  output: \"[#{�_\\a_\\a}]\";\
        \n  output: \"#{�_\\a_\\a}\";\
        \n  output: \"#{�_\\a_\\a}\";\
        \n  output: \"[\'#{�_\\a_\\a}\']\";\
        \n}\
        \n"
    );
}
