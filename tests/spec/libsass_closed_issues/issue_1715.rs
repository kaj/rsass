//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1715.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "div {\
             \n  color: red(blue, purple);\
             \n}"
        ),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 |   color: red(blue, purple);\
         \n  |          ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function red($color) {\
         \n  |           =========== declaration\
         \n  \'\
         \n  input.scss 2:10  root stylesheet",
    );
}
