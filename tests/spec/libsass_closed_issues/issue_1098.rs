//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1098.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n opacity: 1\\9;\
            \n width: 500px\\9;\
            \n color: #f00000\\9\\0;\
            \n color: #f00000\\9\\0\\;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  opacity: 1\\9 ;\
        \n  width: 500px\\9 ;\
        \n  color: #f00000\\9 \\0 ;\
        \n  color: #f00000\\9 \\0 \\;;\
        \n}\
        \n"
    );
}
