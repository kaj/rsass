//! Tests auto-converted from "sass-spec/spec/css/moz_document"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/css/moz_document/empty_prefix.hrx"
#[test]
fn empty_prefix() {
    assert_eq!(
        rsass(
            "// An empty url-prefix() should not be deprecated yet, as it\'s still supported\
            \n// in Firefox\'s release channel at time of writing.\
            \n\
            \n@-moz-document url-prefix() {\
            \n  a {b: c}\
            \n}\
            \n\
            \n@-moz-document url-prefix(\"\") {\
            \n  a {b: c}\
            \n}\
            \n\
            \n@-moz-document url-prefix(\'\') {\
            \n  a {b: c}\
            \n}\
            \n"
        )
        .unwrap(),
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
        \n}\
        \n"
    );
}

mod functions;

// From "sass-spec/spec/css/moz_document/multi_function.hrx"
#[test]
#[ignore] // wrong result
fn multi_function() {
    assert_eq!(
        rsass(
            "@-moz-document url(http://www.w3.org/),\
            \n               url-prefix(http://www.w3.org/Style/),\
            \n               domain(mozilla.org),\
            \n               regexp(\"https:.*\") {\
            \n  a {b: c}\
            \n}\
            \n"
        )
        .unwrap(),
        "@-moz-document url(http://www.w3.org/),\
        \n               url-prefix(http://www.w3.org/Style/),\
        \n               domain(mozilla.org),\
        \n               regexp(\"https:.*\") {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n"
    );
}
