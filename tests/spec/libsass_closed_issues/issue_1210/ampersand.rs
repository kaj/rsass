//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1210/ampersand.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  @at-root {\
            \n    & {\
            \n      color: blue;\
            \n    }\
            \n\
            \n    &--modifier {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  color: blue;\
            \n\
            \n  @at-root {\
            \n    & bar {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  color: blue;\
            \n\
            \n  @at-root {\
            \n    bar & {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  color: blue;\
            \n\
            \n  @at-root {\
            \n    bar {\
            \n        & baz {\
            \n            color: red;\
            \n        }\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n    @at-root bar & {\
            \n        color: red;\
            \n\
            \n        & baz {\
            \n            color: blue;\
            \n        }\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  color: blue;\
        \n}\
        \nfoo--modifier {\
        \n  color: red;\
        \n}\
        \nfoo {\
        \n  color: blue;\
        \n}\
        \nfoo bar {\
        \n  color: red;\
        \n}\
        \nfoo {\
        \n  color: blue;\
        \n}\
        \nbar foo {\
        \n  color: red;\
        \n}\
        \nfoo {\
        \n  color: blue;\
        \n}\
        \nbar baz {\
        \n  color: red;\
        \n}\
        \nbar foo {\
        \n  color: red;\
        \n}\
        \nbar foo baz {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
