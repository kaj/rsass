//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_828.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  box-shadow: inset -1.5em 0 1.5em -0.75em rgba(0, 0, 0, 0.25);\
            \n  box-shadow: inset -1.5em 0 1.5em - 0.75em rgba(0, 0, 0, 0.25);\
            \n  box-shadow: inset -1.5em 0 1.5em- 0.75em rgba(0, 0, 0, 0.25);\
            \n  box-shadow: inset -1.5em 0 1.5em-0.75em rgba(0, 0, 0, 0.25);\
            \n  box-shadow: inset -1.5em 0 1.5em -.75em rgba(0, 0, 0, .25);\
            \n  box-shadow: inset -1.5em 0 1.5em - .75em rgba(0, 0, 0, .25);\
            \n  box-shadow: inset -1.5em 0 1.5em- .75em rgba(0, 0, 0, .25);\
            \n  box-shadow: inset -1.5em 0 1.5em-.75em rgba(0, 0, 0, .25);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  box-shadow: inset -1.5em 0 1.5em -0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 1.5em- 0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 1.5em -0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 1.5em- 0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 0.75em rgba(0, 0, 0, 0.25);\
        \n}\
        \n"
    );
}
