//! Tests auto-converted from "sass-spec/spec/libsass/at-root/135_test_simple_at_root.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("135_test_simple_at_root")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  @at-root {\
             \n    .bar {a: b}\
             \n  }\
             \n}\n"),
        ".bar {\
         \n  a: b;\
         \n}\n"
    );
}
