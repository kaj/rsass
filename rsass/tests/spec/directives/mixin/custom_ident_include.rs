//! Tests auto-converted from "sass-spec/spec/directives/mixin/custom_ident_include.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("custom_ident_include")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin __a() {b: c}\
             \nd {@include --a}\n"
        ),
        "Error: Sass @mixin names beginning with -- are forbidden for forward-compatibility with plain CSS mixins.\n\
         \nFor details, see https://sass-lang.com/d/css-function-mixin\
         \n  ,\
         \n2 | d {@include --a}\
         \n  |             ^^^\
         \n  \'\
         \n  input.scss 2:13  root stylesheet",
    );
}
