//! Tests auto-converted from "sass-spec/spec/css/moz_document/multi_function.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multi_function")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@-moz-document url(http://www.w3.org/),\
             \n               url-prefix(http://www.w3.org/Style/),\
             \n               domain(mozilla.org),\
             \n               regexp(\"https:.*\") {\
             \n  a {b: c}\
             \n}\n"),
        "@-moz-document url(http://www.w3.org/),\
         \n               url-prefix(http://www.w3.org/Style/),\
         \n               domain(mozilla.org),\
         \n               regexp(\"https:.*\") {\
         \n  a {\
         \n    b: c;\
         \n  }\
         \n}\n"
    );
}
