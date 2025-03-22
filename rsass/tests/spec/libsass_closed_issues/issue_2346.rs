//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2346.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2346")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$items: 3;\r\
             \nli {\r\
             \n  &:nth-child(#{$items}n - #{$items}) {\r\
             \n    color: red;\r\
             \n  }\r\
             \n}"),
        "li:nth-child(3n-3) {\
         \n  color: red;\
         \n}\n"
    );
}
