//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-error/ruleset.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("ruleset")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "a {\r\
             \n  @error \"error\";\r\
             \n  foo: bar;\r\
             \n}"
        ),
        "Error: \"error\"\
         \n  ,\
         \n2 |   @error \"error\";\
         \n  |   ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
