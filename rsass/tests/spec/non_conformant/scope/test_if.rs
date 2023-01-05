//! Tests auto-converted from "sass-spec/spec/non_conformant/scope/if.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("if")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: 42;\n\
             \n.foo {\
             \n  content: $foo;\
             \n}\n\
             \n@if true {\
             \n  $foo: 1337 !global;\
             \n}\n\
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
