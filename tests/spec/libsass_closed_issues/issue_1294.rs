//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1294.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "/*------------------------------------*\\\
            \n  #BUTTONS\
            \n\\*------------------------------------*/\
            \n\
            \nfoo {\
            \n  display: inline-block; /* [1] */\
            \n}\
            \n"
        )
        .unwrap(),
        "/*------------------------------------*\\\
        \n  #BUTTONS\
        \n\\*------------------------------------*/\
        \nfoo {\
        \n  display: inline-block;\
        \n  /* [1] */\
        \n}\
        \n"
    );
}
