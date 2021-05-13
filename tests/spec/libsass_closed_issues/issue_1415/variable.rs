//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1415/variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$parent: &;\n\
             \n@if $parent {\
             \n  foo {\
             \n    foo: bar;\
             \n  }\
             \n}\n"),
        ""
    );
}
