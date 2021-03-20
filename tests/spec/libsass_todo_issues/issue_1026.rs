//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1026.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  a {\
            \n    /**\
            \n     * a\
            \n     * multiline\
            \n     * comment\
            \n     */\
            \n    top: 10px;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div a {\
        \n  /**\
        \n   * a\
        \n   * multiline\
        \n   * comment\
        \n   */\
        \n  top: 10px;\
        \n}\
        \n"
    );
}
