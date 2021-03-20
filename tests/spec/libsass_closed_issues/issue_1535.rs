//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1535.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n    test: type-of(1--em);\
            \n    test: (1--em-2--em);\
            \n    test: (1--em- 2--em);\
            \n    test: (1--em -2--em);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  test: list;\
        \n  test: 1 --em-2--em;\
        \n  test: 1 --em- 2 --em;\
        \n  test: 1 --em -2 --em;\
        \n}\
        \n"
    );
}
