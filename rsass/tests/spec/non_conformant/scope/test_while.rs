//! Tests auto-converted from "sass-spec/spec/non_conformant/scope/while.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("while")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: 42;\n\
             \n.foo {\
             \n  content: $foo;\
             \n}\n\
             \n$condition: 1337;\
             \n@while $condition > 0 {\
             \n  $foo: $condition !global;\
             \n  $condition: 0;\
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
