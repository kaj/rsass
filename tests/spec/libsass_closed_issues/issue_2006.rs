//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2006.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin main() {\
             \n  bar {\
             \n    baz: 1;\
             \n  }\
             \n}\n\
             \nfoo {\
             \n  @at-root {\
             \n    @include main();\
             \n  }\
             \n}\n"),
        "bar {\
         \n  baz: 1;\
         \n}\n"
    );
}
