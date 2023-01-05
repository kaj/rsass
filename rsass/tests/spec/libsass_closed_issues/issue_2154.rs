//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2154.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2154")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@media (min-width: 1px) {\
             \n  .first {\
             \n    font-weight: 100;\n\
             \n    @media (min-width: 2px) {}\
             \n  }\
             \n  .second {\
             \n    font-weight: 200;\
             \n  }\
             \n}\n"),
        "@media (min-width: 1px) {\
         \n  .first {\
         \n    font-weight: 100;\
         \n  }\
         \n  .second {\
         \n    font-weight: 200;\
         \n  }\
         \n}\n"
    );
}
