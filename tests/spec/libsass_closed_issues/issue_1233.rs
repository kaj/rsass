//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1233.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@-moz-keyframes animatetoptop /* Firefox */ line 429\
             \n{\
             \nfrom {width:0%}\
             \nto {width:100%}\
             \n}"),
        "@-moz-keyframes animatetoptop /* Firefox */ line 429 {\
         \n  from {\
         \n    width: 0%;\
         \n  }\
         \n  to {\
         \n    width: 100%;\
         \n  }\
         \n}\n"
    );
}
