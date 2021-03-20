//! Tests auto-converted from "sass-spec/spec/libsass/at-root/135_test_simple_at_root.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  @at-root {\
            \n    .bar {a: b}\
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
