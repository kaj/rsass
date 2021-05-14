//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/content/arguments/error/syntax.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod arglist {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn invalid() {
        assert_eq!(
            runner().err(
                "@mixin mixin {\
             \n  @content;\
             \n}\n\
             \n@include mixin() using ($arg1: ) {}\n"
            ),
            "Error: Expected expression.\
         \n  ,\
         \n5 | @include mixin() using ($arg1: ) {}\
         \n  |                                ^\
         \n  \'\
         \n  input.scss 5:32  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn missing() {
        assert_eq!(
            runner().err(
                "@mixin mixin {\
             \n  @content;\
             \n}\n\
             \n@include mixin using {}\n"
            ),
            "Error: expected \"(\".\
         \n  ,\
         \n5 | @include mixin using {}\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 5:22  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn missing_parens() {
        assert_eq!(
            runner().err(
                "@mixin mixin {\
             \n  @content;\
             \n}\n\
             \n@include mixin using $arg1 {}\n"
            ),
            "Error: expected \"(\".\
         \n  ,\
         \n5 | @include mixin using $arg1 {}\
         \n  |                      ^\
         \n  \'\
         \n  input.scss 5:22  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong error
fn missing_block() {
    assert_eq!(
        runner().err(
            "@mixin mixin {\
             \n  @content;\
             \n}\n\
             \n@include mixin using ();\n"
        ),
        "Error: expected \"{\".\
         \n  ,\
         \n5 | @include mixin using ();\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 5:24  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn missing_using() {
    assert_eq!(
        runner().err(
            "@mixin mixin {\
             \n  @content;\
             \n}\n\
             \n@include mixin() () {}\n"
        ),
        "Error: expected \";\".\
         \n  ,\
         \n5 | @include mixin() () {}\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 5:18  root stylesheet",
    );
}
