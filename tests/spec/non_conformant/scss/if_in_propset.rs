//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/if-in-propset.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  prop: {\
            \n    a: \"hello\";\
            \n    b: \"goodbye\";\
            \n    @if true {\
            \n      c: \"badbye\";\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  prop-a: \"hello\";\
        \n  prop-b: \"goodbye\";\
        \n  prop-c: \"badbye\";\
        \n}\
        \n"
    );
}
