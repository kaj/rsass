//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1210/extend.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  @at-root {\
            \n    %placeholder {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n\
            \n  baz {\
            \n    @at-root {\
            \n      %other-placeholder {\
            \n        color: blue;\
            \n      }\
            \n    }\
            \n  }\
            \n\
            \n  %ampersand-placeholder & {\
            \n    color: green;\
            \n  }\
            \n\
            \n  @at-root {\
            \n    qux {\
            \n      @extend %ampersand-placeholder;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nbar {\
            \n  @extend %placeholder;\
            \n}\
            \n\
            \nbaz {\
            \n  @extend %other-placeholder;\
            \n}\
            \n\
            \nbam {\
            \n  @extend %ampersand-placeholder;\
            \n}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  color: red;\
        \n}\
        \nbaz {\
        \n  color: blue;\
        \n}\
        \nbam foo, qux foo {\
        \n  color: green;\
        \n}\
        \n"
    );
}
