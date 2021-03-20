//! Tests auto-converted from "sass-spec/spec/non_conformant/scope/each.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: 42;\
            \n\
            \n.foo {\
            \n  content: $foo;\
            \n}\
            \n\
            \n@each $item in 1337 {\
            \n  $foo: $item !global;\
            \n}\
            \n\
            \n.bar {\
            \n  content: $foo;\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  content: 42;\
        \n}\
        \n.bar {\
        \n  content: 1337;\
        \n}\
        \n"
    );
}
