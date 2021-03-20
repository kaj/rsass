//! Tests auto-converted from "sass-spec/spec/libsass/at-root/138_test_at_root_in_media.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@media screen {\
            \n  .foo {\
            \n    @at-root .bar {a: b}\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}
