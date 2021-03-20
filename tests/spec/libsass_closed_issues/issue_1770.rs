//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1770.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function returns-string() {\
            \n  @return \"selector\";\
            \n}\
            \n\
            \n#{\"selector\"} {\
            \n  color: red;\
            \n}\
            \n\
            \n#{returns-string()} {\
            \n  color: red;\
            \n}\
            \n\
            \n#{\"selector\"} selector2 {\
            \n  color: red;\
            \n}\
            \n\
            \n#{returns-string()} selector2 {\
            \n  color: red;\
            \n}"
        )
        .unwrap(),
        "selector {\
        \n  color: red;\
        \n}\
        \nselector {\
        \n  color: red;\
        \n}\
        \nselector selector2 {\
        \n  color: red;\
        \n}\
        \nselector selector2 {\
        \n  color: red;\
        \n}\
        \n"
    );
}
