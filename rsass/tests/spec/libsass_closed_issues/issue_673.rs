//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_673.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_673")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            ".example {\
             \n    padding-left: 2rem;\
             \n    padding-right: 2rem;\
             \n}\
             \n@media screen and (min-width:768px) {\n\
             \n    #footer {\
             \n        .row {\
             \n            @extend .example;\
             \n        }\
             \n    }\n\
             \n}"
        ),
        "Error: You may not @extend selectors across media queries.\
         \n    ,\
         \n1   | .example {\
         \n    | ======== extended selector\
         \n... |\
         \n5   | @media screen and (min-width:768px) {\
         \n    |        ============================ extension @media\
         \n... |\
         \n9   |             @extend .example;\
         \n    |             ^^^^^^^^^^^^^^^^ extension\
         \n    \'\
         \n  input.scss 9:13  root stylesheet",
    );
}
