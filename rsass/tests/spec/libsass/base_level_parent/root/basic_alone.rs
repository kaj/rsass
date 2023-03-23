//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-alone.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-alone")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "& {\
             \n  foo {\
             \n    bar: baz;\
             \n  }\
             \n}\n"
        ),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | & {\
         \n  | ^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
