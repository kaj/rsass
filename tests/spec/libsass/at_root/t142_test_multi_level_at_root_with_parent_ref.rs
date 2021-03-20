//! Tests auto-converted from "sass-spec/spec/libsass/at-root/142_test_multi_level_at_root_with_parent_ref.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  @at-root & {\
            \n    .bar {\
            \n      @at-root & {\
            \n        a: b;\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo .bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
