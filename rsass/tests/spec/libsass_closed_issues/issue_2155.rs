//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2155.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2155")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "a {\
             \n  &.#{#ff0} {\
             \n    color: red;\
             \n  }\
             \n}\n"
        ),
        "WARNING: You probably don\'t mean to use the color value yellow in interpolation here.\
         \nIt may end up represented as #ff0, which will likely produce invalid CSS.\
         \nAlways quote color names when using them as strings or map keys (for example, \"yellow\").\
         \nIf you really want to use the color value here, use \'\"\" + #ff0\'.\n\
         \n  ,\
         \n2 |   &.#{#ff0} {\
         \n  |       ^^^^\
         \n  \'\
         \n    input.scss 2:7  root stylesheet\n\
         \nError: Expected identifier.\
         \n  ,--> input.scss\
         \n2 |   &.#{#ff0} {\
         \n  |       ^^^^ \
         \n  \'\
         \n  ,\
         \n1 | &.#ff0 \
         \n  |   = error in interpolated output\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
