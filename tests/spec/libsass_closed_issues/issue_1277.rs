//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1277.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: foo;\
             \n$bar: bar;\n\
             \n.foo {\
             \n  foo: foo #{$foo}, bar #{$bar};\
             \n}\n"),
        ".foo {\
         \n  foo: foo foo, bar bar;\
         \n}\n"
    );
}
