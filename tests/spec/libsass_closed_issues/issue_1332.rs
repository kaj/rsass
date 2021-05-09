//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1332.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".box1 {\
             \n    color: rgb(20%, 20%, 20%);\
             \n}\
             \n.box2 {\
             \n    color: rgb(32, 32, 32);\
             \n}\
             \n.box3 {\
             \n    color: rgba(20%, 20%, 20%, 0.7);\
             \n}\n"),
        ".box1 {\
         \n  color: #333333;\
         \n}\
         \n.box2 {\
         \n  color: #202020;\
         \n}\
         \n.box3 {\
         \n  color: rgba(51, 51, 51, 0.7);\
         \n}\n"
    );
}
