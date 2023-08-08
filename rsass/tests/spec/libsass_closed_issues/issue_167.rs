//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_167.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_167")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("%l-cell, .l-cell {\r\
             \n  margin: 0 auto;\r\
             \n  max-width: 1000px;\r\
             \n}"),
        ".l-cell {\
         \n  margin: 0 auto;\
         \n  max-width: 1000px;\
         \n}\n"
    );
}
