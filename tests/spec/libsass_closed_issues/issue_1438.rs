//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1438.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo() {\
            \n  @return 20150812;\
            \n}\
            \n\
            \nfoo {\
            \n  background-image: url(../test.png);\
            \n}\
            \n\
            \nbar {\
            \n  background-image: url(../test.png?v=20150812);\
            \n}\
            \n\
            \nbaz {\
            \n  background-image: url(../test.png?v=#{test()});\
            \n}\
            \n\
            \nbam {\
            \n  background-image: url(\"../test.png?v=#{test()}\");\
            \n}\
            \n"
        )
        .unwrap(),
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
        \n}\
        \n"
    );
}
