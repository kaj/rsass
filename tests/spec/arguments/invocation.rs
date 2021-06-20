//! Tests auto-converted from "sass-spec/spec/arguments/invocation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod function {
    #[allow(unused)]
    use super::runner;

    mod error {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn positional_after_named() {
            assert_eq!(
        runner().err(
            "@function a($b, $c) {@return null}\n\
             \n$d: a($b: 1, 2);\n"
        ),
        "Error: Positional arguments must come before keyword arguments.\
         \n  ,\
         \n3 | $d: a($b: 1, 2);\
         \n  |              ^\
         \n  \'\
         \n  input.scss 3:14  root stylesheet",
    );
        }
    }
}
mod mixin {
    #[allow(unused)]
    use super::runner;

    mod error {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn duplicate_named() {
            assert_eq!(
                runner().err(
                    "@mixin a($b) {}\n\
             \n@include a($b: 1, $b: 2);\n"
                ),
                "Error: Duplicate argument.\
         \n  ,\
         \n3 | @include a($b: 1, $b: 2);\
         \n  |                   ^^\
         \n  \'\
         \n  input.scss 3:19  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn duplicate_named_normalization() {
            assert_eq!(
                runner().err(
                    "@mixin a($b-c) {}\n\
             \n@include a($b-c: 1, $b_c: 2);\n"
                ),
                "Error: Duplicate argument.\
         \n  ,\
         \n3 | @include a($b-c: 1, $b_c: 2);\
         \n  |                     ^^^^\
         \n  \'\
         \n  input.scss 3:21  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn positional_after_named() {
            assert_eq!(
        runner().err(
            "@mixin a($b, $c) {}\n\
             \n@include a($b: 1, 2) {}\n"
        ),
        "Error: Positional arguments must come before keyword arguments.\
         \n  ,\
         \n3 | @include a($b: 1, 2) {}\
         \n  |                   ^\
         \n  \'\
         \n  input.scss 3:19  root stylesheet",
    );
        }
    }
}
