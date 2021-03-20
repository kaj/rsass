//! Tests auto-converted from "sass-spec/spec/libsass/at-root/136_test_at_root_with_selector.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  @at-root .bar {a: b}\
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
