//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2560.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2560")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: 10px / 5px;\r\
             \n\r\
             \ntest {\r\
             \n    font-size: $x;\r\
             \n    font-size: #{$x};\r\
             \n}"),
        "test {\
         \n  font-size: 2;\
         \n  font-size: 2;\
         \n}\n"
    );
}
