//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/double_escape/32_double_escaped_literal.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("32_double_escaped_literal")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".test32#{\'\\\\@baz\'} { content: \'3.2\'; }\n"),
        ".test32\\@baz {\
         \n  content: \"3.2\";\
         \n}\n"
    );
}
