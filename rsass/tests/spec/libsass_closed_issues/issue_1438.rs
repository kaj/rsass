//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1438.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1438")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo() {\
             \n  @return 20150812;\
             \n}\n\
             \nfoo {\
             \n  background-image: url(../test.png);\
             \n}\n\
             \nbar {\
             \n  background-image: url(../test.png?v=20150812);\
             \n}\n\
             \nbaz {\
             \n  background-image: url(../test.png?v=#{test()});\
             \n}\n\
             \nbam {\
             \n  background-image: url(\"../test.png?v=#{test()}\");\
             \n}\n"),
        "foo {\
         \n  background-image: url(../test.png);\
         \n}\
         \nbar {\
         \n  background-image: url(../test.png?v=20150812);\
         \n}\
         \nbaz {\
         \n  background-image: url(../test.png?v=test());\
         \n}\
         \nbam {\
         \n  background-image: url(\"../test.png?v=test()\");\
         \n}\n"
    );
}
