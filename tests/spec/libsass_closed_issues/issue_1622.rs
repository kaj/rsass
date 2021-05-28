//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1622.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($list) {\
             \n    @return call(bar, $list);\
             \n}\n\
             \n@function bar($list, $args...) {\
             \n    @return length($list);\
             \n}\n\
             \ntest {\
             \n  test: foo(1 2 3);\
             \n}\n"),
        "test {\
         \n  test: 3;\
         \n}\n"
    );
}
