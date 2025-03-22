//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2006.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2006")
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
