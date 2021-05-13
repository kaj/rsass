//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_267.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: foo;\r\
             \n@keyframes $x {\r\
             \n  to {\r\
             \n    blah: blah;\r\
             \n  }\r\
             \n}"),
        "@keyframes $x {\
         \n  to {\
         \n    blah: blah;\
         \n  }\
         \n}\n"
    );
}
