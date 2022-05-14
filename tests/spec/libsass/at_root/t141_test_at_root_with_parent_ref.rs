//! Tests auto-converted from "sass-spec/spec/libsass/at-root/141_test_at_root_with_parent_ref.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("141_test_at_root_with_parent_ref")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  @at-root & {\
             \n    a: b;\
             \n  }\
             \n}\n"),
        ".foo {\
         \n  a: b;\
         \n}\n"
    );
}
