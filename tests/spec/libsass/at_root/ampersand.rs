//! Tests auto-converted from "sass-spec/spec/libsass/at-root/ampersand.hrx"

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
            \n"
        )
        .unwrap(),
        "foo {\
        \n  color: blue;\
        \n}\
        \nfoo--modifier {\
        \n  color: red;\
        \n}\
        \n"
    );
}
