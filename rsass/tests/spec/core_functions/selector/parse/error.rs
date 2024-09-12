//! Tests auto-converted from "sass-spec/spec/core_functions/selector/parse/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

#[test]
fn inner_comma() {
    assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.parse(((c,),))}\n"
        ),
        "Error: $selector: ((c,),) is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n2 | a {b: selector.parse(((c,),))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn outer_space() {
    assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \n@use \"sass:selector\";\
             \na {b: selector.parse(list.append((), list.append((), c)))}\n"
        ),
        "Error: $selector: (c) is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n3 | a {b: selector.parse(list.append((), list.append((), c)))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
}
#[test]
fn parent() {
    assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.parse(\"&\")}\n"
        ),
        "Error: $selector: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &\
         \n  | ^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.parse(\"&\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod parse {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn extra() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.parse(\"c {\")}\n"
            ),
            "Error: $selector: expected selector.\
         \n  ,\
         \n1 | c {\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.parse(\"c {\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn invalid() {
        assert_eq!(
            runner().err(
                "@use \"sass:selector\";\
             \na {b: selector.parse(\"[c\")}\n"
            ),
            "Error: $selector: expected more input.\
         \n  ,\
         \n1 | [c\
         \n  |   ^\
         \n  \'\
         \n  - 1:3  root stylesheet\
         \n  ,\
         \n2 | a {b: selector.parse(\"[c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod slash_list {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn in_comma_list() {
        assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \n@use \"sass:list\";\
             \na {b: selector.parse((list.slash(c, d), list.slash(e, f)))}\n"
        ),
        "Error: $selector: (c / d, e / f) is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n3 | a {b: selector.parse((list.slash(c, d), list.slash(e, f)))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn top_level() {
        assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \n@use \"sass:list\";\
             \na {b: selector.parse(list.slash(c d, e f))}\n"
        ),
        "Error: $selector: (c d / e f) is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n3 | a {b: selector.parse(list.slash(c d, e f))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
    }
}
#[test]
fn too_few_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.parse()}\n"
        ),
        "Error: Missing argument $selector.\
         \n  ,--> input.scss\
         \n2 | a {b: selector.parse()}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function parse($selector) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn too_many_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.parse(c, d)}\n"
        ),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: selector.parse(c, d)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:selector\
         \n1 | @function parse($selector) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn too_nested() {
    assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \n@use \"sass:selector\";\
             \na {b: selector.parse((list.append((), list.append((), c)),))}\n"
        ),
        "Error: $selector: (c,) is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n3 | a {b: selector.parse((list.append((), list.append((), c)),))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
    );
}
#[test]
fn test_type() {
    assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \na {b: selector.parse(1)}\n"
        ),
        "Error: $selector: 1 is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n2 | a {b: selector.parse(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn wrong_name() {
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
