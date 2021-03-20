//! Tests auto-converted from "sass-spec/spec/css/moz_document/functions/static.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@-moz-document url(sass-lang.com) {\
            \n  a {type: unquoted url}\
            \n}\
            \n@-moz-document url(\"sass-lang.com\") {\
            \n  a {type: quoted url}\
            \n}\
            \n\
            \n@-moz-document url-prefix(https://sass-lang.com/docs) {\
            \n  a {type: unquoted url-prefix}\
            \n}\
            \n@-moz-document url-prefix(\"https://sass-lang.com/docs\") {\
            \n  a {type: quoted url-prefix}\
            \n}\
            \n\
            \n@-moz-document domain(sass-lang.com) {\
            \n  a {type: unquoted domain}\
            \n}\
            \n@-moz-document domain(\"sass-lang.com\") {\
            \n  a {type: quoted domain}\
            \n}\
            \n\
            \n@-moz-document regexp(\"https:.*\") {\
            \n  a {type: regexp}\
            \n}\
            \n"
        )
        .unwrap(),
        "@-moz-document url(sass-lang.com) {\
        \n  a {\
        \n    type: unquoted url;\
        \n  }\
        \n}\
        \n@-moz-document url(\"sass-lang.com\") {\
        \n  a {\
        \n    type: quoted url;\
        \n  }\
        \n}\
        \n@-moz-document url-prefix(https://sass-lang.com/docs) {\
        \n  a {\
        \n    type: unquoted url-prefix;\
        \n  }\
        \n}\
        \n@-moz-document url-prefix(\"https://sass-lang.com/docs\") {\
        \n  a {\
        \n    type: quoted url-prefix;\
        \n  }\
        \n}\
        \n@-moz-document domain(sass-lang.com) {\
        \n  a {\
        \n    type: unquoted domain;\
        \n  }\
        \n}\
        \n@-moz-document domain(\"sass-lang.com\") {\
        \n  a {\
        \n    type: quoted domain;\
        \n  }\
        \n}\
        \n@-moz-document regexp(\"https:.*\") {\
        \n  a {\
        \n    type: regexp;\
        \n  }\
        \n}\
        \n"
    );
}
