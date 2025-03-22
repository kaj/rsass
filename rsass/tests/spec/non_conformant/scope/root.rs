//! Tests auto-converted from "sass-spec/spec/non_conformant/scope/root.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("root")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: 42;\n\
             \n.foo {\
             \n  content: $foo;\
             \n}\n\
             \n$foo: 1337 !global;\n\
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
