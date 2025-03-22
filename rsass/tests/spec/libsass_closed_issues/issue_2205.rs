//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2205.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2205")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$params1: #fff .5;\r\
             \ntest {\r\
             \n  color: rgba($params1...);\r\
             \n}\r\
             \n\r\
             \n$params2: 10 250 130 .5;\r\
             \ntest {\r\
             \n  color: rgba($params2...);\r\
             \n}"),
        "test {\
         \n  color: rgba(255, 255, 255, 0.5);\
         \n}\
         \ntest {\
         \n  color: rgba(10, 250, 130, 0.5);\
         \n}\n"
    );
}
