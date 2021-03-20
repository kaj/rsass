//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/concat.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  a: hello + \"goodbye\";\
            \n  b: \"hello\" + goodbye;\
            \n  c: 3 + \"hello\";\
            \n  d: \"hello\" + 3;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: hellogoodbye;\
        \n  b: \"hellogoodbye\";\
        \n  c: \"3hello\";\
        \n  d: \"hello3\";\
        \n}\
        \n"
    );
}
