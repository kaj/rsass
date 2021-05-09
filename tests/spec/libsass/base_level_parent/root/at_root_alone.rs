//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-alone.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@at-root {\
             \n  & {\
             \n    foo {\
             \n      bar: baz;\
             \n    }\
             \n  }\
             \n}"
        ),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n2 |   & {\
         \n  |   ^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
