//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/file/loop/each.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file("_include.scss", "")
}

#[test]
#[ignore] // missing error
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
