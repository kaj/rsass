//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2366/has.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("has")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".item {\r\
             \n    display: inline-block;\r\
             \n\r\
             \n    :has(> .ext-link) {\r\
             \n        background: #000;\r\
             \n    }\r\
             \n}\r\
             \n\r\
             \n.link {\r\
             \n    color: red;\r\
             \n}\r\
             \n\r\
             \n.textLink {\r\
             \n    @extend .link;\r\
             \n    padding: 0 10px;\r\
             \n}"),
        ".item {\
         \n  display: inline-block;\
         \n}\
         \n.item :has(> .ext-link) {\
         \n  background: #000;\
         \n}\
         \n.link, .textLink {\
         \n  color: red;\
         \n}\
         \n.textLink {\
         \n  padding: 0 10px;\
         \n}\n"
    );
}
