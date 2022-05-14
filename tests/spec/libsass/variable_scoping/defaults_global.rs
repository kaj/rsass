//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/defaults-global.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("defaults-global")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  $foo: inner !default !global;\
             \n  $foo: lexical;\
             \n  inner { foo: $foo; }\
             \n}\n\
             \n$foo: outer !default !global;\
             \nouter { foo: $foo; }\n\
             \ndiv {\
             \n  $foo: footer !default !global;\
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
