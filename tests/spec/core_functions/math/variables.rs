//! Tests auto-converted from "sass-spec/spec/core_functions/math/variables.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn e() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.$e}\n"),
        "a {\
         \n  b: 2.7182818285;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    mod assignment {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn e() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\" as math;\
             \nmath.$e: 0;\n"
                ),
                "Error: Cannot modify built-in variable.\
         \n  ,\
         \n2 | math.$e: 0;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn pi() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\" as math;\
             \nmath.$pi: 0;\n"
                ),
                "Error: Cannot modify built-in variable.\
         \n  ,\
         \n2 | math.$pi: 0;\
         \n  | ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
            );
        }
    }
}
#[test]
fn pi() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.$pi}\n"),
        "a {\
         \n  b: 3.1415926536;\
         \n}\n"
    );
}
