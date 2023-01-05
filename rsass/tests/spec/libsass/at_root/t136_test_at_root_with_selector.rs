//! Tests auto-converted from "sass-spec/spec/libsass/at-root/136_test_at_root_with_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("136_test_at_root_with_selector")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  @at-root .bar {a: b}\
             \n}\n"),
        ".bar {\
         \n  a: b;\
         \n}\n"
    );
}
