//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/return-in-mixin.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("return-in-mixin")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@mixin mix() {\r\
             \n  @return 42;\r\
             \n}\r\
             \n\r\
             \nfoo {\r\
             \n  @include mix();\r\
             \n}"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @return 42;\
         \n  |   ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
