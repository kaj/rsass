//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod extendee {
    #[allow(unused)]
    use super::runner;

    mod complex {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn list() {
            assert_eq!(
                runner().err("a {b: selector-extend(\"c\", d e, \"f\")}\n"),
                "Error: Can\'t extend complex selector d e.\
         \n  ,\
         \n1 | a {b: selector-extend(\"c\", d e, \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn string() {
            assert_eq!(
                runner()
                    .err("a {b: selector-extend(\"c\", \"d e\", \"f\")}\n"),
                "Error: Can\'t extend complex selector d e.\
         \n  ,\
         \n1 | a {b: selector-extend(\"c\", \"d e\", \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn invalid() {
        assert_eq!(
            runner().err("a {b: selector-extend(\"c\", \"[d\", \"e\")}\n"),
            "Error: $extendee: expected more input.\
         \n  ,\
         \n1 | [d\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-extend(\"c\", \"[d\", \"e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn parent() {
        assert_eq!(
            runner().err("a {b: selector-extend(\"c\", \"&\", \"d\")}\n"),
            "Error: $extendee: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-extend(\"c\", \"&\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_type() {
        assert_eq!(
        runner().err(
            "a {b: selector-extend(\"c\", 1, \"d\")}\n"
        ),
        "Error: $extendee: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-extend(\"c\", 1, \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
mod extender {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn invalid() {
        assert_eq!(
            runner().err("a {b: selector-extend(\"c\", \"d\", \"[e\")}\n"),
            "Error: $extender: expected more input.\
         \n  ,\
         \n1 | [e\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-extend(\"c\", \"d\", \"[e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn parent() {
        assert_eq!(
            runner().err("a {b: selector-extend(\"c\", \"d\", \"&\")}\n"),
            "Error: $extender: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-extend(\"c\", \"d\", \"&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_type() {
        assert_eq!(
        runner().err(
            "a {b: selector-extend(\"c\", \"d\", 1)}\n"
        ),
        "Error: $extender: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-extend(\"c\", \"d\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
mod selector {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn invalid() {
        assert_eq!(
            runner().err("a {b: selector-extend(\"[c\", \"d\", \"e\")}\n"),
            "Error: $selector: expected more input.\
         \n  ,\
         \n1 | [c\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-extend(\"[c\", \"d\", \"e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn parent() {
        assert_eq!(
            runner().err("a {b: selector-extend(\"&\", \"c\", \"d\")}\n"),
            "Error: $selector: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-extend(\"&\", \"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_type() {
        assert_eq!(
        runner().err(
            "a {b: selector-extend(1, \"c\", \"d\")}\n"
        ),
        "Error: $selector: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-extend(1, \"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
#[test]
#[ignore] // missing error
fn too_few_args() {
    assert_eq!(
        runner().err("a {b: selector-extend(\"c\", \"d\")}\n"),
        "Error: Missing argument $extender.\
         \n  ,--> input.scss\
         \n1 | a {b: selector-extend(\"c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function extend($selector, $extendee, $extender) {\
         \n  |           ======================================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn too_many_args() {
    assert_eq!(
        runner().err("a {b: selector-extend(\"c\", \"d\", \"e\", \"f\")}\n"),
        "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: selector-extend(\"c\", \"d\", \"e\", \"f\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function extend($selector, $extendee, $extender) {\
         \n  |           ======================================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
