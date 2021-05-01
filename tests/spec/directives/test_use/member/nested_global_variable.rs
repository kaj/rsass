//! Tests auto-converted from "sass-spec/spec/directives/use/member/nested_global_variable.hrx"

#[test]
#[ignore] // unexepected error
fn direct() {
    assert_eq!(
        crate::rsass(
            "@use \"other\";\
            \n\
            \na {b: inspect(other.$member)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: null;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn through_import() {
    assert_eq!(
        crate::rsass(
            "@use \"used\";\
            \n\
            \na {b: inspect(used.$member)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: null;\
        \n}\
        \n"
    );
}
