//! Tests auto-converted from "sass-spec/spec/css/important.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
    use super::runner;
    mod syntax {
        #[allow(unused)]
        use super::runner;
        #[test]
        #[ignore] // wrong error
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
