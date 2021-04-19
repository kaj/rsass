//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/miss/loop/for.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@for $i from 1 through 2 {\r\
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
         \n  input.scss 2:3  root stylesheet\
         \n",
    );
}
