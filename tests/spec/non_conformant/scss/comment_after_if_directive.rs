//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/comment_after_if_directive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  @if true {a: b}\
            \n  @else {x: y}\
            \n  /* This is a comment */\
            \n  c: d }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: b;\
        \n  /* This is a comment */\
        \n  c: d;\
        \n}\
        \n"
    );
}
