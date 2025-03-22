//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/miss/control-if.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("control-if")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@if (true) {\r\
             \n  @import \'_include\';\r\
             \n}\r\n"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @import \'_include\';\
         \n  |   ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
