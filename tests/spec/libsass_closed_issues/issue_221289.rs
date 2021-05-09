//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_221289.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\r\
             \n  bar: if(0,0<0,0);\r\
             \n}"),
        "foo {\
         \n  bar: false;\
         \n}\n"
    );
}
