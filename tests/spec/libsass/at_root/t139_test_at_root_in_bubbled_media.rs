//! Tests auto-converted from "sass-spec/spec/libsass/at-root/139_test_at_root_in_bubbled_media.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  @media screen {\
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
