//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/double_escape/12_double_escaped_interpolated_value_todo.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$key: \'bar\';\
            \n.test12#{\'\\\\@#{$key}\'} { content: \'1.2\'; }\
            \n"
        )
        .unwrap(),
        ".test12\\@bar {\
        \n  content: \"1.2\";\
        \n}\
        \n"
    );
}
