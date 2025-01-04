//! Tests auto-converted from "sass-spec/spec/values/calculation/calc-size.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("calc-size")
}

#[test]
#[ignore] // wrong result
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: CaLc-size(auto, size - 20px)}\n"),
        "a {\
         \n  b: calc-size(auto, size - 20px);\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn sass_script() {
        assert_eq!(
            runner().err("a {b: calc-size(auto, 7 % 3)}\n"),
            "Error: This operation can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: calc-size(auto, 7 % 3)}\
         \n  |                         ^\
         \n  \'\
         \n  input.scss 1:25  root stylesheet",
        );
    }
    mod syntax {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: calc-size(auto, $)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: calc-size(auto, $)}\
         \n  |                        ^\
         \n  \'\
         \n  input.scss 1:24  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: calc-size()}\n"),
            "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: calc-size()}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: calc-size(auto, 0, 0)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,\
         \n1 | a {b: calc-size(auto, 0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn one_arg() {
    assert_eq!(
        runner().ok("a {b: calc-size(var(--foo))}\n"),
        "a {\
         \n  b: calc-size(var(--foo));\
         \n}\n"
    );
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function calc-size($arg) {@return $arg}\
             \na {b: calc-size(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simplified() {
    assert_eq!(
        runner().ok("a {b: calc-size(auto, 100px - 20px + size)}\n"),
        "a {\
         \n  b: calc-size(auto, 80px + size);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn unsimplified() {
    assert_eq!(
        runner().ok("a {b: calc-size(auto, 5% - 20px + size)}\n"),
        "a {\
         \n  b: calc-size(auto, 5% - 20px + size);\
         \n}\n"
    );
}
