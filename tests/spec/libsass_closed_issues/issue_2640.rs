//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue-2640.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".theme1, .theme2 {\
            \n  .something {\
            \n    /* nothing */\
            \n  }\
            \n}\
            \n\
            \n$sel: selector-nest(\'.theme1, .theme2\', \'.something\');\
            \n  \
            \n#{$sel} {\
            \n  /* nothing */\
            \n}\
            \n"
        )
        .unwrap(),
        ".theme1 .something, .theme2 .something {\
        \n  /* nothing */\
        \n}\
        \n.theme1 .something, .theme2 .something {\
        \n  /* nothing */\
        \n}\
        \n"
    );
}
