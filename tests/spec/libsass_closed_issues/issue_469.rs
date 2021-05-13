//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_469.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("/*!\
             \n*/\n\
             \n@charset \"utf-8\";\n\
             \na {\
             \n  color: red;\
             \n}\n\
             \n@import url(\"x\");\n"),
        "/*!\
         \n*/\
         \n@import url(\"x\");\
         \na {\
         \n  color: red;\
         \n}\n"
    );
}
