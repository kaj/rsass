//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_6.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("*[class|=\"has-background\"] {\r\
             \n    background: #efefef;\r\
             \n    padding: 7px;\r\
             \n    border: 1px solid #888;\r\
             \n    margin-bottom: 5px;\r\
             \n    }"),
        "*[class|=has-background] {\
         \n  background: #efefef;\
         \n  padding: 7px;\
         \n  border: 1px solid #888;\
         \n  margin-bottom: 5px;\
         \n}\n"
    );
}
