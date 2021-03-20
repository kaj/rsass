//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/double_escape/22_double_escaped_interpolated_variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$key: \'bar\';\
            \n$suffix2: \'\\\\@#{$key}\';\
            \n.test22#{$suffix2} { content: \'2.2\'; }\
            \n"
        )
        .unwrap(),
        ".test22\\@bar {\
        \n  content: \"2.2\";\
        \n}\
        \n"
    );
}
