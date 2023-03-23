//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2482.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2482")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin mixin()\
             \n{\
             \n\t& .class\
             \n\t{\
             \n\t\tcolor: black;\
             \n\t}\
             \n}\n\
             \n@include mixin();"
        ),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n3 |     & .class\
         \n  |     ^\
         \n  \'\
         \n  input.scss 3:2  mixin()\
         \n  input.scss 9:1  root stylesheet",
    );
}
