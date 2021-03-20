//! Tests auto-converted from "sass-spec/spec/libsass/at-root/143_test_multi_level_at_root_with_inner_parent_ref.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  @at-root .bar {\
            \n    @at-root & {\
            \n      a: b;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
