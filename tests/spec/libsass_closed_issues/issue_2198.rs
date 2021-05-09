//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2198.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin test() {\
             \n  @at-root {\
             \n    @include foo();\
             \n  }\
             \n}\n\
             \n@mixin foo() {\
             \n  #{\'.foo\'} {\
             \n    bar: baz;\
             \n  }\
             \n}\n\
             \n.test {\
             \n  @include test();\
             \n}\n"),
        ".foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
