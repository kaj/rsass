//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_empty_declarations.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_empty_declarations")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: baz; }\n"),
        "foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
