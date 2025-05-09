//! Tests auto-converted from "sass-spec/spec/core_functions/selector/nest/error.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod invalid {
    use super::runner;

    #[test]
    fn initial() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.nest(\"[c\")}\n"
            ),
            "Error: expected more input.\
         \n  ,\
         \n1 | [c\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.nest(\"[c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn later() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"[d\")}\n"
            ),
            "Error: expected more input.\
         \n  ,\
         \n1 | [d\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.nest(\"c\", \"[d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod parent {
    use super::runner;

    #[test]
    fn first_arg() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.nest(\"&\")}\n"
            ),
            "Error: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.nest(\"&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn non_initial() {
        assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"[d]&\")}\n"
        ),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n1 | [d]&\
         \n  |    ^\
         \n  \'\
         \n  - 1:4  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.nest(\"c\", \"[d]&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn prefix() {
        assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", \"d&\")}\n"
        ),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n1 | d&\
         \n  |  ^\
         \n  \'\
         \n  - 1:2  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.nest(\"c\", \"d&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
#[test]
fn too_few_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.nest()}\n\n"
        ),
        "Error: $selectors: At least one selector must be passed.\
         \n  ,\
         \n2 | a {b: selector.nest()}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod test_type {
    use super::runner;

    #[test]
    fn initial() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.nest(1)}\n"
            ),
            "Error: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n2 | a {b: selector.nest(1)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn later() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", 1)}\n"
            ),
            "Error: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n2 | a {b: selector.nest(\"c\", 1)}\
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
            "@use \"sass:selector\";\
             \na {b: selector.selector-nest(c, d)}\n"
        ),
        "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: selector.selector-nest(c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
