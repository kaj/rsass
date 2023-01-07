//! Tests auto-converted from "sass-spec/spec/non_conformant/nesting/parent_with_newline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("parent_with_newline")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo,\
             \n.bar {\
             \n  .baz & {\
             \n    color: red;\
             \n  }\
             \n}\n"),
        ".baz .foo,\
         \n.baz .bar {\
         \n  color: red;\
         \n}\n"
    );
}
