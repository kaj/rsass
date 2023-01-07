//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/defaults-global-null.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("defaults-global-null")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  $foo: null !default !global;\
             \n  $foo: inner !default !global;\
             \n  $foo: null !default !global;\
             \n  $foo: lexical;\
             \n  inner { foo: $foo; }\
             \n}\n\
             \n$foo: null !default !global;\
             \n$foo: outer !default !global;\
             \n$foo: null !default !global;\
             \nouter { foo: $foo; }\n\
             \ndiv {\
             \n  $foo: null !default !global;\
             \n  $foo: footer !default !global;\
             \n  $foo: null !default !global;\
             \n  $foo: lexical;\
             \n  inner { foo: $foo; }\
             \n}\n"),
        "div inner {\
         \n  foo: lexical;\
         \n}\
         \nouter {\
         \n  foo: inner;\
         \n}\
         \ndiv inner {\
         \n  foo: lexical;\
         \n}\n"
    );
}
