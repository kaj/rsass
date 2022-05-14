//! Tests auto-converted from "sass-spec/spec/non_conformant/scope/each.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("each")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: 42;\n\
             \n.foo {\
             \n  content: $foo;\
             \n}\n\
             \n@each $item in 1337 {\
             \n  $foo: $item !global;\
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
