//! Tests auto-converted from "sass-spec/spec/core_functions/string/slice/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod decimal {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn end() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.slice(\"\", 1, 1.5)}\n"
            ),
            "Error: 1.5 is not an int.\
         \n  ,\
         \n2 | a {b: string.slice(\"\", 1, 1.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn start() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.slice(\"\", 0.5)}\n"
            ),
            "Error: 0.5 is not an int.\
         \n  ,\
         \n2 | a {b: string.slice(\"\", 0.5)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn too_few_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:string\";\
             \na {b: string.slice(\"cde\")}\n"
        ),
        "Error: Missing argument $start-at.\
         \n  ,--> input.scss\
         \n2 | a {b: string.slice(\"cde\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function slice($string, $start-at, $end-at: -1) {\
         \n  |           ====================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn too_many_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:string\";\
             \na {b: string.slice(\"cde\", 1, 2, 3)}\n"
        ),
        "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: string.slice(\"cde\", 1, 2, 3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function slice($string, $start-at, $end-at: -1) {\
         \n  |           ====================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod test_type {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn end_at() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.slice(\"cde\", 1, \"f\")}\n"
            ),
            "Error: $end-at: \"f\" is not a number.\
         \n  ,\
         \n2 | a {b: string.slice(\"cde\", 1, \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn start_at() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.slice(\"cde\", \"f\")}\n"
            ),
            "Error: $start-at: \"f\" is not a number.\
         \n  ,\
         \n2 | a {b: string.slice(\"cde\", \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn string() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.slice(1, 2)}\n"
            ),
            "Error: $string: 1 is not a string.\
         \n  ,\
         \n2 | a {b: string.slice(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod unit {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn end() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.slice(\"\", 1, 2px)}\n"
            ),
            "Error: $end-at: Expected 2px to have no units.\
         \n  ,\
         \n2 | a {b: string.slice(\"\", 1, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn start() {
        assert_eq!(
            runner().err(
                "@use \"sass:string\";\
             \na {b: string.slice(\"\", 1px)}\n"
            ),
            "Error: $start-at: Expected 1px to have no units.\
         \n  ,\
         \n2 | a {b: string.slice(\"\", 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn wrong_name() {
    assert_eq!(
        runner().err(
            "@use \"sass:string\";\
             \na {b: string.str-slice(\"c\", 1, 1)}\n"
        ),
        "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: string.str-slice(\"c\", 1, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
