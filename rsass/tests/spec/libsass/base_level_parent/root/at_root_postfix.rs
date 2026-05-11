//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-postfix.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-root-postfix")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@at-root {\
             \n  &post {\
             \n    foo {\
             \n      bar: baz;\
             \n    }\
             \n  }\
             \n}"
        ),
        "Error: A top-level selector may not contain a parent selector with a suffix.\
         \n  ,\
         \n2 |   &post {\
         \n  |   ^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
