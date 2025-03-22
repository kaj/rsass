//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_622.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_622")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media screen {\
             \n    a {\
             \n        color: red;\
             \n    }\
             \n}\n\
             \n.link {\
             \n    @media (foo: bar) {\
             \n        display: flex;\
             \n    }\
             \n}\n"),
        "@media screen {\
         \n  a {\
         \n    color: red;\
         \n  }\
         \n}\
         \n@media (foo: bar) {\
         \n  .link {\
         \n    display: flex;\
         \n  }\
         \n}\n"
    );
}
