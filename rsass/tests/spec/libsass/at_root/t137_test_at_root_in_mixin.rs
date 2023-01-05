//! Tests auto-converted from "sass-spec/spec/libsass/at-root/137_test_at_root_in_mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("137_test_at_root_in_mixin")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin bar {\
             \n  @at-root .bar {a: b}\
             \n}\n\
             \n.foo {\
             \n  @include bar;\
             \n}\n"),
        ".bar {\
         \n  a: b;\
         \n}\n"
    );
}
