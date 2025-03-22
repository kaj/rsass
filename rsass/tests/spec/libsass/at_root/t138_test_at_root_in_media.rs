//! Tests auto-converted from "sass-spec/spec/libsass/at-root/138_test_at_root_in_media.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("138_test_at_root_in_media")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media screen {\
             \n  .foo {\
             \n    @at-root .bar {a: b}\
             \n  }\
             \n}\n"),
        "@media screen {\
         \n  .bar {\
         \n    a: b;\
         \n  }\
         \n}\n"
    );
}
