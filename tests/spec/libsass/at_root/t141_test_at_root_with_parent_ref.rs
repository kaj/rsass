//! Tests auto-converted from "sass-spec/spec/libsass/at-root/141_test_at_root_with_parent_ref.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  @at-root & {\
            \n    a: b;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
