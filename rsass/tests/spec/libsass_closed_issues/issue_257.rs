//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_257.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_257")
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
