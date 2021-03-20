//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1567.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "/* any */@media/* first */\
            \n/* screen */screen /*something */ , /* else */\
            \n/* not */not/* print */print /* final */ {  /* whatever */\
            \n    body { line-height: 1.2 }\
            \n}\
            \n"
        )
        .unwrap(),
        "/* any */\
        \n@media screen, not print {\
        \n  /* whatever */\
        \n  body {\
        \n    line-height: 1.2;\
        \n  }\
        \n}\
        \n"
    );
}
