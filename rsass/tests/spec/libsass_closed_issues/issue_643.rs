//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_643.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_643")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n$map: (foo: bar, bar: baz);\n\
             \nfoo {\
             \n  a: list.nth($map, 2);\
             \n}\n"),
        "foo {\
         \n  a: bar baz;\
         \n}\n"
    );
}
