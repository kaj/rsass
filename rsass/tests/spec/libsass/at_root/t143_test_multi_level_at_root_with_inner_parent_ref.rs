//! Tests auto-converted from "sass-spec/spec/libsass/at-root/143_test_multi_level_at_root_with_inner_parent_ref.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("143_test_multi_level_at_root_with_inner_parent_ref")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  @at-root .bar {\
             \n    @at-root & {\
             \n      a: b;\
             \n    }\
             \n  }\
             \n}\n"),
        ".bar {\
         \n  a: b;\
         \n}\n"
    );
}
