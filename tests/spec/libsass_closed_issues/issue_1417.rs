//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1417.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1417")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@function foo($a, $b) {\
             \n  @return ($a $b);\
             \n}\n\
             \nfoo {\
             \n  foo: 1px / 2px;\
             \n  foo: 1px / round(1.5);\
             \n  foo: foo(1px / 2px, 1px / round(1.5));\
             \n  foo: missing(1px / 2px, 1px / round(1.5));\
             \n  foo: call(missing, 1px / 2px, 1px / round(1.5));\
             \n}\n"),
        "foo {\
         \n  foo: 1px/2px;\
         \n  foo: 0.5px;\
         \n  foo: 0.5 0.5px;\
         \n  foo: missing(1px/2px, 0.5px);\
         \n  foo: missing(0.5, 0.5px);\
         \n}\n"
    );
}
