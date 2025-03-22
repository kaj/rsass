//! Tests auto-converted from "sass-spec/spec/libsass/at-root/139_test_at_root_in_bubbled_media.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("139_test_at_root_in_bubbled_media")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  @media screen {\
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
