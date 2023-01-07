//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

#[test]
fn too_few_args() {
    assert_eq!(
        runner().err("a {a: inspect()}\n"),
        "Error: Missing argument $value.\
         \n  ,--> input.scss\
         \n1 | a {a: inspect()}\
         \n  |       ^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function inspect($value) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn too_many_args() {
    assert_eq!(
        runner().err("a {a: inspect(1, 2)}\n"),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {a: inspect(1, 2)}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function inspect($value) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
