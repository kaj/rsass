//! Tests auto-converted from "sass-spec/spec/css/mixin.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin")
}

mod error {
    use super::runner;

    mod css {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn mixin() {
            assert_eq!(
                runner().err("@mixin --a {}\n"),
                "Error: Sass @mixin names beginning with -- are forbidden for forward-compatibility with plain CSS mixins.\n\
         \nFor details, see https://sass-lang.com/d/css-function-mixin\
         \n  ,\
         \n1 | @mixin --a {}\
         \n  |        ^^^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
            );
        }
    }
}
