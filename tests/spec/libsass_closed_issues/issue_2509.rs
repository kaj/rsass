//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2509.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("[charset i] {\r\
             \n\tdisplay: block;\r\
             \n}\r\
             \n\r\
             \n[charset I] {\r\
             \n\tdisplay: block;\r\
             \n}\r\
             \n\r\
             \n[charset=\"utf-8\" i] {\r\
             \n\tdisplay: block;\r\
             \n}\r\
             \n\r\
             \n[charset=\"utf-8\" I] {\r\
             \n\tdisplay: block;\r\
             \n}"),
        "[charset i] {\
         \n  display: block;\
         \n}\
         \n[charset I] {\
         \n  display: block;\
         \n}\
         \n[charset=\"utf-8\" i] {\
         \n  display: block;\
         \n}\
         \n[charset=\"utf-8\" I] {\
         \n  display: block;\
         \n}\n"
    );
}
