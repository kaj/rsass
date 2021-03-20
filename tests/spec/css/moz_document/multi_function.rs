//! Tests auto-converted from "sass-spec/spec/css/moz_document/multi_function.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
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
