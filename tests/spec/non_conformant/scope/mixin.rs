//! Tests auto-converted from "sass-spec/spec/non_conformant/scope/mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: 42;\n\
             \n@mixin foo {\
             \n  $foo: 1337 !global;\
             \n}\n\
             \n.foo {\
             \n  content: $foo;\
             \n}\n\
             \n@include foo;\n\
             \n.bar {\
             \n  content: $foo;\
             \n}\n"),
        ".foo {\
         \n  content: 42;\
         \n}\
         \n.bar {\
         \n  content: 1337;\
         \n}\n"
    );
}
