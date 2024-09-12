//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod selector1 {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn invalid() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.unify(\"[c\", \"d\")}\n"
            ),
            "Error: $selector1: expected more input.\
         \n  ,\
         \n1 | [c\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.unify(\"[c\", \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn parent() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.unify(\"&\", \"c\")}\n"
            ),
            "Error: $selector1: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.unify(\"&\", \"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.unify(1, \"c\")}\n"
        ),
        "Error: $selector1: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n2 | a {b: selector.unify(1, \"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
mod selector2 {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn invalid() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.unify(\"c\", \"[d\")}\n"
            ),
            "Error: $selector2: expected more input.\
         \n  ,\
         \n1 | [d\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.unify(\"c\", \"[d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn parent() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.unify(\"c\", \"&\")}\n"
            ),
            "Error: $selector2: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.unify(\"c\", \"&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.unify(\"c\", 1)}\n"
        ),
        "Error: $selector2: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n2 | a {b: selector.unify(\"c\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: selector.selector-unify(\".c\", \".d\")}\n"
        ),
        "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: selector.selector-unify(\".c\", \".d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
