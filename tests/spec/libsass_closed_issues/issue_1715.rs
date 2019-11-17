//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1715.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
             \n  color: red(blue, purple);\
             \n}"
        )
        .unwrap_err(),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 |   color: red(blue, purple);\
         \n  |          ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function red($color) {\
         \n  |           =========== declaration\
         \n  \'\
         \n  input.scss 2:10  root stylesheet\
         \n",
    );
}
