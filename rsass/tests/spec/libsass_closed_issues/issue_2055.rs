//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2055.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2055")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
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
             \n}\n"
        ),
        ":not(.thing):not(:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled]))))) {\
         \n  color: red;\
         \n}\
         \n:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled]))))):not([disabled]:has(:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled]))))))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled]))))):not([disabled]:has(:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))))))) {\
         \n  background: blue;\
         \n}\
         \n:has(:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled]))))):not([disabled]:has(:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled]))))))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled]))))):not([disabled]:has(:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled]))))))))) {\
         \n  background: blue;\
         \n}\n"
    );
}
