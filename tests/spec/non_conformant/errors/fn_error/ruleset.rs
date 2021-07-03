//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-error/ruleset.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
