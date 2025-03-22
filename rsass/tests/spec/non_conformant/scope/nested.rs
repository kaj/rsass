//! Tests auto-converted from "sass-spec/spec/non_conformant/scope/nested.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nested")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: 42;\n\
             \n.foo {\
             \n  content: $foo;\n\
             \n  > .bar {\
             \n    $foo: 1337 !global;\
             \n  }\
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
