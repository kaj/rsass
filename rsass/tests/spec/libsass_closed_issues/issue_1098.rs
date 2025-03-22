//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1098.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1098")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n opacity: 1\\9;\
             \n width: 500px\\9;\
             \n color: #f00000\\9\\0;\
             \n color: #f00000\\9\\0\\;\
             \n}\n"),
        "div {\
         \n  opacity: 1\\9 ;\
         \n  width: 500px\\9 ;\
         \n  color: #f00000\\9 \\0 ;\
         \n  color: #f00000\\9 \\0 \\;;\
         \n}\n"
    );
}
