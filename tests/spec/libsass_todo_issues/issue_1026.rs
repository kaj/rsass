//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1026.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
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
