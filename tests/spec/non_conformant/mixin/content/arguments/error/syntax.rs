//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/content/arguments/error/syntax.hrx"

mod arglist {
    #[test]
    #[ignore] // wrong error
    fn invalid() {
        assert_eq!(
            crate::rsass(
                "@mixin mixin {\
             \n  @content;\
             \n}\
             \n\
             \n@include mixin() using ($arg1: ) {}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@mixin mixin {\
             \n  @content;\
             \n}\
             \n\
             \n@include mixin using {}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@mixin mixin {\
             \n  @content;\
             \n}\
             \n\
             \n@include mixin using $arg1 {}\
             \n"
            )
            .unwrap_err(),
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
        crate::rsass(
            "@mixin mixin {\
             \n  @content;\
             \n}\
             \n\
             \n@include mixin using ();\
             \n"
        )
        .unwrap_err(),
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
        crate::rsass(
            "@mixin mixin {\
             \n  @content;\
             \n}\
             \n\
             \n@include mixin() () {}\
             \n"
        )
        .unwrap_err(),
        "Error: expected \";\".\
         \n  ,\
         \n5 | @include mixin() () {}\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 5:18  root stylesheet",
    );
}
