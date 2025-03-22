//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1596.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1596")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@document url(http://www.w3.org/),\
             \n               url-prefix(http://www.w3.org/Style/),\
             \n               domain(mozilla.org),\
             \n               regexp(\"https:.*\");\n"),
        "@document url(http://www.w3.org/),\
         \n               url-prefix(http://www.w3.org/Style/),\
         \n               domain(mozilla.org),\
         \n               regexp(\"https:.*\");\n"
    );
}
