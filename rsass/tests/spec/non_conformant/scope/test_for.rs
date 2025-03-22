//! Tests auto-converted from "sass-spec/spec/non_conformant/scope/for.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("for")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: 42;\n\
             \n.foo {\
             \n  content: $foo;\
             \n}\n\
             \n@for $i from 1337 to 1338 {\
             \n  $foo: $i !global;\
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
