//! Tests auto-converted from "sass-spec/spec/css/important.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("important")
}

mod error {
    use super::runner;

    mod syntax {
        use super::runner;

        #[test]
        fn eof_after_bang() {
            assert_eq!(
        runner().err(
            "// Regression test for sass/dart-sass#1049. The lack of a trailing newline is\
             \n// necessary for the regression test.\
             \na {b: !"
        ),
        "Error: Expected \"important\".\
         \n  ,\
         \n3 | a {b: !\
         \n  |        ^\
         \n  \'\
         \n  input.scss 3:8  root stylesheet",
    );
        }
    }
}
mod syntax {
    use super::runner;

    mod escaped {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn after_bang() {
            assert_eq!(
                runner().ok("a {b: c !\\69 mportant}\n"),
                "a {\
         \n  b: c !important;\
         \n}\n"
            );
        }
    }
}
