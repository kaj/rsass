//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/extend/selector/optional.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("optional")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".baz {\r\
             \n  @extend .foo !optional;\r\
             \n  color: green;\r\
             \n}\r\n"),
        ".baz {\
         \n  color: green;\
         \n}\n"
    );
}
