//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2055.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ":not(.thing) {\
            \n    color: red;\
            \n}\
            \n:not(.thing[disabled]) {\
            \n    @extend .thing;\
            \n    background: blue;\
            \n}\
            \n:has(:not(.thing[disabled])) {\
            \n    @extend .thing;\
            \n    background: blue;\
            \n}\
            \n"
        )
        .unwrap(),
        ":not(.thing) {\
        \n  color: red;\
        \n}\
        \n:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled]))))) {\
        \n  background: blue;\
        \n}\
        \n:has(:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))))) {\
        \n  background: blue;\
        \n}\
        \n"
    );
}
