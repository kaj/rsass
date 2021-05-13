//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_invisible_comments.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  a: d; }\n"),
        "foo {\
         \n  a: d;\
         \n}\n"
    );
}
