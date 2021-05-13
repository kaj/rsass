//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_308.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$var: orange;\n\
             \n.test {\
             \n  color: $var;\
             \n}\n\
             \n.#{$var} {\
             \n  color: #C0362C;\
             \n}\n"),
        ".test {\
         \n  color: orange;\
         \n}\
         \n.orange {\
         \n  color: #C0362C;\
         \n}\n"
    );
}
