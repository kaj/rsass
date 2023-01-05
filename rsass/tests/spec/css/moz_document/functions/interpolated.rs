//! Tests auto-converted from "sass-spec/spec/css/moz_document/functions/interpolated.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("interpolated")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@-moz-document url(#{\"sass-lang.com\"}) {\
             \n  a {type: unquoted full url}\
             \n}\
             \n@-moz-document url(#{sa + ss}-lang.com) {\
             \n  a {type: unquoted partial url}\
             \n}\
             \n@-moz-document url(\"#{sa + ss}-lang.com\") {\
             \n  a {type: quoted partial url}\
             \n}\n\
             \n@-moz-document url-prefix(#{\"https://sass-lang.com/docs\"}) {\
             \n  a {type: unquoted full url-prefix}\
             \n}\
             \n@-moz-document url-prefix(#{ht + tps}://sass-lang.com/docs) {\
             \n  a {type: unquoted partial url-prefix}\
             \n}\
             \n@-moz-document url-prefix(\"#{ht + tps}://sass-lang.com/docs\") {\
             \n  a {type: quoted partial url-prefix}\
             \n}\n\
             \n@-moz-document domain(#{\"sass-lang.com\"}) {\
             \n  a {type: unquoted full domain}\
             \n}\
             \n@-moz-document domain(#{sa + ss}-lang.com) {\
             \n  a {type: unquoted partial domain}\
             \n}\
             \n@-moz-document domain(\"#{sa + ss}-lang.com\") {\
             \n  a {type: quoted partial domain}\
             \n}\n\
             \n@-moz-document regexp(\"#{ht + tp}s:.*\") {\
             \n  a {type: regexp}\
             \n}\n"
        ),
        "@-moz-document url(sass-lang.com) {\
         \n  a {\
         \n    type: unquoted full url;\
         \n  }\
         \n}\
         \n@-moz-document url(sass-lang.com) {\
         \n  a {\
         \n    type: unquoted partial url;\
         \n  }\
         \n}\
         \n@-moz-document url(\"sass-lang.com\") {\
         \n  a {\
         \n    type: quoted partial url;\
         \n  }\
         \n}\
         \n@-moz-document url-prefix(https://sass-lang.com/docs) {\
         \n  a {\
         \n    type: unquoted full url-prefix;\
         \n  }\
         \n}\
         \n@-moz-document url-prefix(https://sass-lang.com/docs) {\
         \n  a {\
         \n    type: unquoted partial url-prefix;\
         \n  }\
         \n}\
         \n@-moz-document url-prefix(\"https://sass-lang.com/docs\") {\
         \n  a {\
         \n    type: quoted partial url-prefix;\
         \n  }\
         \n}\
         \n@-moz-document domain(sass-lang.com) {\
         \n  a {\
         \n    type: unquoted full domain;\
         \n  }\
         \n}\
         \n@-moz-document domain(sass-lang.com) {\
         \n  a {\
         \n    type: unquoted partial domain;\
         \n  }\
         \n}\
         \n@-moz-document domain(\"sass-lang.com\") {\
         \n  a {\
         \n    type: quoted partial domain;\
         \n  }\
         \n}\
         \n@-moz-document regexp(\"https:.*\") {\
         \n  a {\
         \n    type: regexp;\
         \n  }\
         \n}\n"
    );
}
