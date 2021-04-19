//! Tests auto-converted from "sass-spec/spec/core_functions/selector/parse/error.hrx"

#[test]
#[ignore] // missing error
fn inner_comma() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-parse(((c,),))}\
             \n"
        ).unwrap_err(),
        "Error: $selector: ((c,),) is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-parse(((c,),))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn outer_space() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-parse(append((), append((), c)))}\
             \n"
        )
        .unwrap_err(),
        "Error: $selector: c is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-parse(append((), append((), c)))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn parent() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-parse(\"&\")}\
             \n"
        )
        .unwrap_err(),
        "Error: $selector: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-parse(\"&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
mod parse {
    #[test]
    #[ignore] // wrong error
    fn extra() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-parse(\"c {\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $selector: expected selector.\
         \n  ,\
         \n1 | c {\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-parse(\"c {\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn invalid() {
        assert_eq!(
            crate::rsass(
                "a {b: selector-parse(\"[c\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $selector: expected more input.\
         \n  ,\
         \n1 | [c\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n1 | a {b: selector-parse(\"[c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn too_few_args() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-parse()}\
             \n"
        )
        .unwrap_err(),
        "Error: Missing argument $selector.\
         \n  ,--> input.scss\
         \n1 | a {b: selector-parse()}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function parse($selector) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn too_many_args() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-parse(c, d)}\
             \n"
        )
        .unwrap_err(),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: selector-parse(c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function parse($selector) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn too_nested() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-parse((append((), append((), c)),))}\
             \n"
        ).unwrap_err(),
        "Error: $selector: (c,) is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-parse((append((), append((), c)),))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn test_type() {
    assert_eq!(
        crate::rsass(
            "a {b: selector-parse(1)}\
             \n"
        )
        .unwrap_err(),
        "Error: $selector: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | a {b: selector-parse(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
