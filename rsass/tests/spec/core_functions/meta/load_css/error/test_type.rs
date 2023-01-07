//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/type.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("type")
}

#[test]
fn url() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \n@include meta.load-css(1);\n"
        ),
        "Error: $url: 1 is not a string.\
         \n  ,\
         \n2 | @include meta.load-css(1);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
mod with {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn key() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (1: null));\n"
            ),
            "Error: $with key: 1 is not a string.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (1: null));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
    #[test]
    fn map() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: 1);\n"
            ),
            "Error: $with: 1 is not a map.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: 1);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
}
