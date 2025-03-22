//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_893.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_893")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$gutter: 20px;\n\
             \n.row {\
             \n  margin: 0 -$gutter;\
             \n}"),
        ".row {\
         \n  margin: -20px;\
         \n}\n"
    );
}
