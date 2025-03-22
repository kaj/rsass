//! Tests auto-converted from "sass-spec/spec/libsass/at-root/140_test_at_root_in_unknown_directive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("140_test_at_root_in_unknown_directive")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@fblthp {\
             \n  .foo {\
             \n    @at-root .bar {a: b}\
             \n  }\
             \n}\n"),
        "@fblthp {\
         \n  .bar {\
         \n    a: b;\
         \n  }\
         \n}\n"
    );
}
