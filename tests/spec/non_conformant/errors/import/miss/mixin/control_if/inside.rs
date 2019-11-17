//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/miss/mixin/control-if/inside.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin do_import() {\r\
             \n  @if (true) {\r\
             \n    @import \'_include\';\r\
             \n  }\r\
             \n}\r\
             \n\r\
             \nfoo {\r\
             \n  @include do_import();\r\
             \n}"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n3 |     @import \'_include\';\
         \n  |     ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:5  root stylesheet\
         \n",
    );
}
