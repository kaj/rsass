//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1298.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1298")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@import url(//fonts.googleapis.com/css?family=Roboto:400,500,700,400italic);\
             \nhtml {\
             \n  font-family: roboto, arial, helvetica, sans-serif;\
             \n}\n"
        ),
        "@import url(//fonts.googleapis.com/css?family=Roboto:400,500,700,400italic);\
         \nhtml {\
         \n  font-family: roboto, arial, helvetica, sans-serif;\
         \n}\n"
    );
}
