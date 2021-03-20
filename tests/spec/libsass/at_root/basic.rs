//! Tests auto-converted from "sass-spec/spec/libsass/at-root/basic.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  color: blue;\
            \n\
            \n  @at-root {\
            \n    bar {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  color: blue;\
            \n\
            \n  @at-root bar {\
            \n    color: red;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  color: blue;\
        \n}\
        \nbar {\
        \n  color: red;\
        \n}\
        \nfoo {\
        \n  color: blue;\
        \n}\
        \nbar {\
        \n  color: red;\
        \n}\
        \n"
    );
}
