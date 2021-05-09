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
        #[ignore] // missing error
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
        #[ignore] // missing error
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
