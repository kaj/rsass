//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/defaults-null.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  $foo: null !default;\
            \n  $foo: inner !default;\
            \n  $foo: null !default;\
            \n  $foo: lexical;\
            \n  inner { foo: $foo; }\
            \n}\
            \n\
            \n// this should error\
            \n// empty { foo: $foo; }\
            \n\
            \n$foo: null !default;\
            \n$foo: outer !default;\
            \n$foo: null !default;\
            \nouter { foo: $foo; }\
            \n\
            \ndiv {\
            \n  $foo: null !default;\
            \n  $foo: footer !default;\
            \n  $foo: null !default;\
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
        \n  foo: outer;\
        \n}\
        \ndiv inner {\
        \n  foo: lexical;\
        \n}\
        \n"
    );
}
