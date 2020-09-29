//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/error"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1169/error/color.hrx"

// Ignoring "color", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1169/error/functioncall.hrx"

// Ignoring "functioncall", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1169/error/interpolate.hrx"

// Ignoring "interpolate", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1169/error/simple.hrx"
#[test]
fn simple() {
    assert_eq!(
        rsass(
            "$map: (\r\
            \n  red: \'bar\',\r\
            \n  #{red}: \'baz\',\r\
            \n);\r\
            \n\r\
            \n.foo {\r\
            \n  content: inspect($map);\r\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: (red: \"bar\", red: \"baz\");\
        \n}\
        \n"
    );
}
