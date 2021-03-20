//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_478.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: \"x\";\
            \n$y: \"y\";\
            \n#{$x}--#{$y} {\
            \n  a: 1\
            \n}\
            \n"
        )
        .unwrap(),
        "x--y {\
        \n  a: 1;\
        \n}\
        \n"
    );
}
