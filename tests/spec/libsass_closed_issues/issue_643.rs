//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_643.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$map: (foo: bar, bar: baz);\n\
             \nfoo {\
             \n  a: nth($map, 2);\
             \n}\n"),
        "foo {\
         \n  a: bar baz;\
         \n}\n"
    );
}
