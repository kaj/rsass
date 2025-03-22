//! Tests auto-converted from "sass-spec/spec/css/moz_document/empty_prefix.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("empty_prefix")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "// An empty url-prefix() should not be deprecated yet, as it\'s still supported\
             \n// in Firefox\'s release channel at time of writing.\n\
             \n@-moz-document url-prefix() {\
             \n  a {b: c}\
             \n}\n\
             \n@-moz-document url-prefix(\"\") {\
             \n  a {b: c}\
             \n}\n\
             \n@-moz-document url-prefix(\'\') {\
             \n  a {b: c}\
             \n}\n"
        ),
        "@-moz-document url-prefix() {\
         \n  a {\
         \n    b: c;\
         \n  }\
         \n}\
         \n@-moz-document url-prefix(\"\") {\
         \n  a {\
         \n    b: c;\
         \n  }\
         \n}\
         \n@-moz-document url-prefix(\"\") {\
         \n  a {\
         \n    b: c;\
         \n  }\
         \n}\n"
    );
}
