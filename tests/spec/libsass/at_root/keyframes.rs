//! Tests auto-converted from "sass-spec/spec/libsass/at-root/keyframes.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  color: red;\
            \n\
            \n  @at-root {\
            \n    @keyframes animation {\
            \n      to { color: red; }\
            \n    }\
            \n  }\
            \n\
            \n  bar {\
            \n    color: blue;\
            \n\
            \n    @at-root {\
            \n      @keyframes other-animation {\
            \n        to { color: blue; }\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  color: red;\
        \n}\
        \n@keyframes animation {\
        \n  to {\
        \n    color: red;\
        \n  }\
        \n}\
        \nfoo bar {\
        \n  color: blue;\
        \n}\
        \n@keyframes other-animation {\
        \n  to {\
        \n    color: blue;\
        \n  }\
        \n}\
        \n"
    );
}
