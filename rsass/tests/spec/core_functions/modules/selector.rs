//! Tests auto-converted from "sass-spec/spec/core_functions/modules/selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector")
}

#[test]
fn append() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.append(c, d)}\n"),
        "a {\
         \n  b: cd;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn selector_append() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.selector-append(c, d)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: selector.selector-append(c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn selector_extend() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.selector-extend(c, c, d)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: selector.selector-extend(c, c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn selector_nest() {
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
    #[test]
    fn selector_parse() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.selector-parse(\".c.d\")}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: selector.selector-parse(\".c.d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn selector_replace() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.selector-replace(c, c, d)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: selector.selector-replace(c, c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn selector_unify() {
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
}
#[test]
#[ignore] // unexepected error
fn extend() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(c, c, d)}\n"),
        "a {\
         \n  b: c, d;\
         \n}\n"
    );
}
#[test]
fn is_superselector() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(c, d)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn nest() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(c, d)}\n"),
        "a {\
         \n  b: c d;\
         \n}\n"
    );
}
#[test]
fn parse() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\".c, .d\")}\n"),
        "a {\
         \n  b: .c, .d;\
         \n}\n"
    );
}
#[test]
fn replace() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.replace(c, c, d)}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn simple_selectors() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.simple-selectors(\".c.d\")}\n"),
        "a {\
         \n  b: .c, .d;\
         \n}\n"
    );
}
#[test]
fn unify() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c\", \".d\")}\n"),
        "a {\
         \n  b: .c.d;\
         \n}\n"
    );
}
