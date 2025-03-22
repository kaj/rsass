//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_crazy_comments.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_crazy_comments")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("/* This is a CSS comment. */\
             \n.one {\
             \n  color: green; }\n\
             \n/* Another comment */\
             \n/* The following should not be used:\
             \n.two {color: red;} */\
             \n.three {\
             \n  color: green;\
             \n  /* color: red; */ }\n\
             \n/**\
             \n.four {color: red;} */\
             \n.five {\
             \n  color: green; }\n\
             \n/**/\
             \n.six {\
             \n  color: green; }\n\
             \n/*********/\
             \n.seven {\
             \n  color: green; }\n\
             \n/* a comment **/\
             \n.eight {\
             \n  color: green; }\n"),
        "/* This is a CSS comment. */\
         \n.one {\
         \n  color: green;\
         \n}\
         \n/* Another comment */\
         \n/* The following should not be used:\
         \n.two {color: red;} */\
         \n.three {\
         \n  color: green;\
         \n  /* color: red; */\
         \n}\
         \n/**\
         \n.four {color: red;} */\
         \n.five {\
         \n  color: green;\
         \n}\
         \n/**/\
         \n.six {\
         \n  color: green;\
         \n}\
         \n/*********/\
         \n.seven {\
         \n  color: green;\
         \n}\
         \n/* a comment **/\
         \n.eight {\
         \n  color: green;\
         \n}\n"
    );
}
