//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_257.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("body{background:blue; a{color:black;}}"),
        "body {\
         \n  background: blue;\
         \n}\
         \nbody a {\
         \n  color: black;\
         \n}\n"
    );
}
