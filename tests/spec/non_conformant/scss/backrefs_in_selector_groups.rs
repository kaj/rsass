//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/backrefs-in-selector-groups.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  &:c, & d {\
            \n    hey: ho;\
            \n  }\
            \n}\
            \n\
            \na b {\
            \n  &:c, & d {\
            \n    hey: ho;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a:c, a d {\
        \n  hey: ho;\
        \n}\
        \na b:c, a b d {\
        \n  hey: ho;\
        \n}\
        \n"
    );
}
