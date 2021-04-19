//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/file/control-if.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@if (true) {\r\
             \n  @import \'_include\';\r\
             \n}\r\
             \n"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @import \'_include\';\
         \n  |   ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
