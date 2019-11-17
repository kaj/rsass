//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/miss/mixin/control-if/outside.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin do_import() {\r\
             \n  @import \'_include\';\r\
             \n}\r\
             \n\r\
             \n@if (true) {\r\
             \n  @include do_import();\r\
             \n}"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @import \'_include\';\
         \n  |   ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet\
         \n",
    );
}
