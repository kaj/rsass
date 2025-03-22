//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/miss/loop/while.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("while")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "$count: 0;\r\
             \n@while ($count < 1) {\r\
             \n  @import \'_include\';\r\
             \n  $count: $count + 1;\r\
             \n}\r\n"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n3 |   @import \'_include\';\
         \n  |   ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:3  root stylesheet",
    );
}
