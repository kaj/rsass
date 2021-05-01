//! Tests auto-converted from "sass-spec/spec/directives/use/member/global.hrx"

#[test]
#[ignore] // unexepected error
fn function() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" as *;\
            \n\
            \na {b: member()}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: value;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn mixin() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" as *;\
            \n\
            \n@include member;\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn multiple() {
    assert_eq!(
        crate::rsass(
            "@use \"left\" as *;\
            \n@use \"right\" as *;\
            \n\
            \na {\
            \n  left: $left;\
            \n  right: $right;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  left: left;\
        \n  right: right;\
        \n}\
        \n"
    );
}
mod no_conflict {
    #[test]
    #[ignore] // unexepected error
    fn function() {
        assert_eq!(
            crate::rsass(
                "@use \"other\" as *;\
            \n@use \"other\" as *;\
            \n\
            \na {b: c()}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: d;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn mixin() {
        assert_eq!(
            crate::rsass(
                "@use \"other\" as *;\
            \n@use \"other\" as *;\
            \n\
            \na {@include b}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  c: d;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn variable() {
        assert_eq!(
            crate::rsass(
                "@use \"other\" as *;\
            \n@use \"other\" as *;\
            \n\
            \na {b: $c}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: d;\
        \n}\
        \n"
        );
    }
}
mod variable_assignment {
    #[test]
    #[ignore] // unexepected error
    fn nested() {
        assert_eq!(
        crate::rsass(
            "@use \"other\" as *;\
            \n\
            \na {\
            \n  // A nested variable assignment that doesn\'t have a namespace but is !global\
            \n  // assigns to a global module\'s variable if one exists.\
            \n  $member: new value !global;\
            \n\
            \n  b: get-member();\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: new value;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn top_level() {
        assert_eq!(
            crate::rsass(
                "@use \"other\" as *;\
            \n\
            \n$member: new value;\
            \n\
            \na {b: get-member()}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: new value;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn variable_use() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" as *;\
            \n\
            \na {b: $member}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: value;\
        \n}\
        \n"
    );
}
