//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1026.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1026")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  a {\
             \n    /**\
             \n     * a\
             \n     * multiline\
             \n     * comment\
             \n     */\
             \n    top: 10px;\
             \n  }\
             \n}\n"),
        "div a {\
         \n  /**\
         \n   * a\
         \n   * multiline\
         \n   * comment\
         \n   */\
         \n  top: 10px;\
         \n}\n"
    );
}
