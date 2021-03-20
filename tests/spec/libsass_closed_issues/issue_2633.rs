//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2633.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "$sel1: \'.something__child + .something__child--mod1\';\
            \n$sel2: \'.something__child ~ .something__child--mod2\';\
            \n$result1: selector-unify($sel1, $sel2);\
            \n\
            \n#{$result1} {\
            \n  /* nothing */\
            \n}\
            \n\
            \n.a {\
            \n  color: blue;\
            \n  & > * {\
            \n    @at-root #{selector-unify(&, \'.b\')} {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \n.a, .b {\
            \n  color: blue;\
            \n  & > * {\
            \n    @at-root #{selector-unify(&, \'.c, .d\')} {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".something__child + .something__child--mod1.something__child--mod2 {\
        \n  /* nothing */\
        \n}\
        \n.a {\
        \n  color: blue;\
        \n}\
        \n.a > .b {\
        \n  color: red;\
        \n}\
        \n.a, .b {\
        \n  color: blue;\
        \n}\
        \n.a > .c, .a > .d, .b > .c, .b > .d {\
        \n  color: red;\
        \n}\
        \n"
    );
}
