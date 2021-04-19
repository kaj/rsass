//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/file/control-else.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@if (false) {\r\
             \n} @else {\r\
             \n  @import \'_include\';\r\
             \n}\r\
             \n"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n3 |   @import \'_include\';\
         \n  |   ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:3  root stylesheet",
    );
}
