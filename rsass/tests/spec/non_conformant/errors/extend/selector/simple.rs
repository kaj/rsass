//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/extend/selector/simple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {color: blue}\r\
             \n.bar {color: red}\r\
             \n.baz {\r\
             \n  @extend .foo;\r\
             \n  color: green;\r\
             \n}\r\n"),
        ".foo, .baz {\
         \n  color: blue;\
         \n}\
         \n.bar {\
         \n  color: red;\
         \n}\
         \n.baz {\
         \n  color: green;\
         \n}\n"
    );
}
