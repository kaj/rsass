//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/placeholder-with-media.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("placeholder-with-media")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%a {\
             \n  @media only screen and (max-width: 100px) {\
             \n    color: red;\
             \n  }\
             \n}\n\
             \nb {\
             \n  @extend %a;\
             \n}\n"),
        "@media only screen and (max-width: 100px) {\
         \n  b {\
         \n    color: red;\
         \n  }\
         \n}\n"
    );
}
