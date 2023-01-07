//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1898.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1898")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media (min-width: 640px) { \
             \n  /* comment */\
             \n}\n\
             \ndiv {\
             \n  @media (min-width: 320px) { \
             \n    /* comment */\
             \n  }\
             \n}"),
        "@media (min-width: 640px) {\
         \n  /* comment */\
         \n}\
         \n@media (min-width: 320px) {\
         \n  div {\
         \n    /* comment */\
         \n  }\
         \n}\n"
    );
}
