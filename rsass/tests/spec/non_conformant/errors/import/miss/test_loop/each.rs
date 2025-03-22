//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/miss/loop/each.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("each")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@each $i in (1) {\r\
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
