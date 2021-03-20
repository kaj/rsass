//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/defaults-global-null.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  $foo: null !default !global;\
            \n  $foo: inner !default !global;\
            \n  $foo: null !default !global;\
            \n  $foo: lexical;\
            \n  inner { foo: $foo; }\
            \n}\
            \n\
            \n$foo: null !default !global;\
            \n$foo: outer !default !global;\
            \n$foo: null !default !global;\
            \nouter { foo: $foo; }\
            \n\
            \ndiv {\
            \n  $foo: null !default !global;\
            \n  $foo: footer !default !global;\
            \n  $foo: null !default !global;\
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
