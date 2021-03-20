//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_784.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  @each $item in (a: 1, b: 2, c: 3) {\
            \n    each: $item;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  each: a 1;\
        \n  each: b 2;\
        \n  each: c 3;\
        \n}\
        \n"
    );
}
