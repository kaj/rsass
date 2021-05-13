//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1080.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("/** comment 1 */\
             \n@import url(\"import-1\");\
             \n/** comment 2 */\
             \n@import url(\"import-2\");\
             \n/** comment 3 */\
             \nfoo { bar: baz; }\n"),
        "/** comment 1 */\
         \n@import url(\"import-1\");\
         \n/** comment 2 */\
         \n@import url(\"import-2\");\
         \n/** comment 3 */\
         \nfoo {\
         \n  bar: baz;\
         \n}\n"
    );
}
