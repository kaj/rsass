//! Tests auto-converted from "sass-spec/spec/directives/use/member/use_to_import.hrx"

#[test]
#[ignore] // unexepected error
fn function() {
    assert_eq!(
        crate::rsass(
            "@use \"midstream\";\
            \n\
            \na {b: midstream.member()}\
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
            "@use \"midstream\";\
            \n\
            \n@include midstream.member;\
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
fn variable_assignment() {
    assert_eq!(
        crate::rsass(
            "@use \"midstream\";\
            \n\
            \nmidstream.$member: new value;\
            \n\
            \na {b: midstream.get-member()}\
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
fn variable_use() {
    assert_eq!(
        crate::rsass(
            "@use \"midstream\";\
            \n\
            \na {b: midstream.$member}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: value;\
        \n}\
        \n"
    );
}
