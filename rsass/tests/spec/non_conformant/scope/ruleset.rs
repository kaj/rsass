//! Tests auto-converted from "sass-spec/spec/non_conformant/scope/ruleset.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("ruleset")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: 42;\n\
             \n.foo {\
             \n  content: $foo;\
             \n  $foo: 1337 !global;\
             \n  content: $foo;\
             \n}\n\
             \n.bar {\
             \n  content: $foo;\
             \n}\n"),
        ".foo {\
         \n  content: 42;\
         \n  content: 1337;\
         \n}\
         \n.bar {\
         \n  content: 1337;\
         \n}\n"
    );
}
