//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-postfix.hrx"

#[allow(unused)]
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
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n2 |   &post {\
         \n  |   ^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
