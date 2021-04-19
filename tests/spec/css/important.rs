//! Tests auto-converted from "sass-spec/spec/css/important.hrx"

mod error {
    mod syntax {
        #[test]
        #[ignore] // wrong error
        fn eof_after_bang() {
            assert_eq!(
        crate::rsass(
            "// Regression test for sass/dart-sass#1049. The lack of a trailing newline is\
             \n// necessary for the regression test.\
             \na {b: !"
        ).unwrap_err(),
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
