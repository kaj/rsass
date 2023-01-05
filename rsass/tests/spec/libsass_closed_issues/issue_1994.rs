//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1994.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1994")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%hoverbrighter {\
             \n    &:hover,\
             \n    &:focus {\
             \n        opacity: .8;\n\
             \n        @supports (filter: brightness(120%)) {\
             \n            filter: brightness(120%);\
             \n        }\
             \n    }\
             \n}\n\
             \n.productportal-link {\
             \n    @extend %hoverbrighter;\
             \n}"),
        ".productportal-link:hover, .productportal-link:focus {\
         \n  opacity: 0.8;\
         \n}\
         \n@supports (filter: brightness(120%)) {\
         \n  .productportal-link:hover, .productportal-link:focus {\
         \n    filter: brightness(120%);\
         \n  }\
         \n}\n"
    );
}
