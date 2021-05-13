//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2365.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "a {\r\
             \n    b {\r\
             \n        color: red;\r\
             \n    },\r\
             \n    c {\r\
             \n        color: blue;\r\
             \n    }\r\
             \n}"
        ),
        "Error: expected selector.\
         \n  ,\
         \n4 |     },\
         \n  |      ^\
         \n  \'\
         \n  input.scss 4:6  root stylesheet",
    );
}
