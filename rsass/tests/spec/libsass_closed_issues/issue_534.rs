//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_534.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_534")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \n$foo: (\
             \n    1: foo1 bar1,\
             \n    10: foo2 bar2,\
             \n    100: foo3 bar3,\
             \n);\n\
             \ndiv {\
             \n    foo: map.get($foo, 1);\
             \n    foo: map.get($foo, 10);\
             \n    foo: map.get($foo, 100);\
             \n}\n"),
        "div {\
         \n  foo: foo1 bar1;\
         \n  foo: foo2 bar2;\
         \n  foo: foo3 bar3;\
         \n}\n"
    );
}
