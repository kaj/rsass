//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/miss/mixin/control-if/outside.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("outside")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin do_import() {\r\
             \n  @import \'_include\';\r\
             \n}\r\
             \n\r\
             \n@if (true) {\r\
             \n  @include do_import();\r\
             \n}"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @import \'_include\';\
         \n  |   ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
