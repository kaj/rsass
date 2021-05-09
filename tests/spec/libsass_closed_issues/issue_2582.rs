//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2582.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".test {\r\
             \n  font-size: (16px / 16px) + 0em;\r\
             \n  font-size: (16px / 16px  + 0em);\r\
             \n  font-size: 16px / 16px  + 0em;\r\
             \n}"),
        ".test {\
         \n  font-size: 1em;\
         \n  font-size: 1em;\
         \n  font-size: 1em;\
         \n}\n"
    );
}
