//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_943.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_943")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%dog {\
             \n    @media (min-width: 10px) {\
             \n        &:hover {\
             \n            display: none;\
             \n        }\
             \n    }\
             \n}\n\
             \n.puppy {\
             \n    @extend %dog;\
             \n    background-color: red;\
             \n}"),
        "@media (min-width: 10px) {\
         \n  .puppy:hover {\
         \n    display: none;\
         \n  }\
         \n}\
         \n.puppy {\
         \n  background-color: red;\
         \n}\n"
    );
}
