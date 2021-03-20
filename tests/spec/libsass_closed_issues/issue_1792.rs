//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1792.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\
            \n  test1: (3px*4in) / 1in;\
            \n  test2: ((1px*2in) + (3px*4in)) / 1in;\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  test1: 0.125in;\
        \n  test2: 0.1458333333in;\
        \n}\
        \n"
    );
}
