//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1459.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1459")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@font-face {\r\
             \n  font-family: \"Font Name\";\r\
             \n  src: local(\"Arial\");\r\
             \n  unicode-range: U+270C;\r\
             \n}"),
        "@font-face {\
         \n  font-family: \"Font Name\";\
         \n  src: local(\"Arial\");\
         \n  unicode-range: U+270C;\
         \n}\n"
    );
}
