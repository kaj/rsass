//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2341.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@function aFunction() {\r\
             \n    @return 1em;\r\
             \n}\r\
             \n\r\
             \n@media (max-width: 1em) {\r\
             \n    %placeholder {\r\
             \n        color: red;\r\
             \n    }\r\
             \n}\r\
             \n\r\
             \n@media (max-width: aFunction()) {\r\
             \n    .test {\r\
             \n        @extend %placeholder;\r\
             \n    }\r\
             \n}"),
        "@media (max-width: 1em) {\
         \n  .test {\
         \n    color: red;\
         \n  }\
         \n}\n"
    );
}
