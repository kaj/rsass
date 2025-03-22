//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1323.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1323")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@import url(foo.css) only screen;\
             \n@import url(foo.css) (min-width:400px);\
             \n@import url(foo.css) (min-width:400px) and (max-width:599px);\n"
        ),
        "@import url(foo.css) only screen;\
         \n@import url(foo.css) (min-width: 400px);\
         \n@import url(foo.css) (min-width: 400px) and (max-width: 599px);\n"
    );
}
