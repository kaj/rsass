//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/double_escape/32_double_escaped_literal.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".test32#{\'\\\\@baz\'} { content: \'3.2\'; }\
            \n"
        )
        .unwrap(),
        ".test32\\@baz {\
        \n  content: \"3.2\";\
        \n}\
        \n"
    );
}
