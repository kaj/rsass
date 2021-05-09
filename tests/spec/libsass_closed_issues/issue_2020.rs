//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2020.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("form {\r\
             \n  $selector: nth(&, 1);\r\
             \n  sel: inspect($selector);\r\
             \n  $selector: nth($selector, 1);\r\
             \n  sel: inspect($selector);\r\
             \n} "),
        "form {\
         \n  sel: form;\
         \n  sel: form;\
         \n}\n"
    );
}
