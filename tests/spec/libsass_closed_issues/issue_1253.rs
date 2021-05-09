//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1253.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: bar;\
             \n@keyframes $foo {\
             \n  from { a: b }\
             \n  to { a: c }\
             \n}"),
        "@keyframes $foo {\
         \n  from {\
         \n    a: b;\
         \n  }\
         \n  to {\
         \n    a: c;\
         \n  }\
         \n}\n"
    );
}
