//! Tests auto-converted from "sass-spec/spec/arguments/invocation.hrx"

mod function {
    mod error {
        #[test]
        #[ignore] // missing error
        fn positional_after_named() {
            assert_eq!(
        crate::rsass(
            "@function a($b, $c) {@return null}\
             \n\
             \n$d: a($b: 1, 2);\
             \n"
        ).unwrap_err(),
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
    mod error {
        #[test]
        #[ignore] // missing error
        fn positional_after_named() {
            assert_eq!(
        crate::rsass(
            "@mixin a($b, $c) {}\
             \n\
             \n@include a($b: 1, 2) {}\
             \n"
        ).unwrap_err(),
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
