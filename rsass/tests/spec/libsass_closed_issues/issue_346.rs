//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_346.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_346")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$mediaquery: \'and (min-width: 300px)\';\n\
             \n@media all #{$mediaquery} {\
             \n  div {\
             \n    display: block;\
             \n  }\
             \n}\n"),
        "@media all and (min-width: 300px) {\
         \n  div {\
         \n    display: block;\
         \n  }\
         \n}\n"
    );
}
