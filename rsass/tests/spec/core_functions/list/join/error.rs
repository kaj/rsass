//! Tests auto-converted from "sass-spec/spec/core_functions/list/join/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

#[test]
fn named() {
    assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: list.join(c, d, $invalid: true)}\n"
        ),
        "Error: No parameter named $invalid.\
         \n  ,--> input.scss\
         \n2 | a {b: list.join(c, d, $invalid: true)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function join($list1, $list2, $separator: auto, $bracketed: auto) {\
         \n  |           ======================================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn positional_and_named() {
    assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: list.join(c, d, comma, true, false, $invalid: true)}\n"
        ),
        "Error: Only 4 positional arguments allowed, but 5 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: list.join(c, d, comma, true, false, $invalid: true)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function join($list1, $list2, $separator: auto, $bracketed: auto) {\
         \n  |           ======================================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn too_few_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: list.join(c)}\n"
        ),
        "Error: Missing argument $list2.\
         \n  ,--> input.scss\
         \n2 | a {b: list.join(c)}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function join($list1, $list2, $separator: auto, $bracketed: auto) {\
         \n  |           ======================================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
fn too_many_args() {
    assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: list.join(c, d, comma, true, false)}\n"
        ),
        "Error: Only 4 arguments allowed, but 5 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: list.join(c, d, comma, true, false)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function join($list1, $list2, $separator: auto, $bracketed: auto) {\
         \n  |           ======================================================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
mod test_type {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn separator() {
        assert_eq!(
            runner().err(
                "@use \"sass:list\";\
             \na {b: list.join(c, d, $separator: 1)}\n"
            ),
            "Error: $separator: 1 is not a string.\
         \n  ,\
         \n2 | a {b: list.join(c, d, $separator: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn unknown_separator() {
    assert_eq!(
        runner().err(
            "@use \"sass:list\";\
             \na {b: list.join(c, d, $separator: e)}\n"
        ),
        "Error: $separator: Must be \"space\", \"comma\", \"slash\", or \"auto\".\
         \n  ,\
         \n2 | a {b: list.join(c, d, $separator: e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
