//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/defaults-global.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  $foo: inner !default !global;\
            \n  $foo: lexical;\
            \n  inner { foo: $foo; }\
            \n}\
            \n\
            \n$foo: outer !default !global;\
            \nouter { foo: $foo; }\
            \n\
            \ndiv {\
            \n  $foo: footer !default !global;\
            \n  $foo: lexical;\
            \n  inner { foo: $foo; }\
            \n}\
            \n"
        )
        .unwrap(),
        "div inner {\
        \n  foo: lexical;\
        \n}\
        \nouter {\
        \n  foo: inner;\
        \n}\
        \ndiv inner {\
        \n  foo: lexical;\
        \n}\
        \n"
    );
}
