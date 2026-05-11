//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-postfix.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-postfix")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "&post {\
             \n  foo {\
             \n    bar: baz;\
             \n  }\
             \n}"
        ),
        "Error: A top-level selector may not contain a parent selector with a suffix.\
         \n  ,\
         \n1 | &post {\
         \n  | ^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
