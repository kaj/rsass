//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1283.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: map-merge((1 2: 3), (2 1: 3));\
            \n\
            \n.test {\
            \n  test: inspect($map);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  test: (1 2: 3, 2 1: 3);\
        \n}\
        \n"
    );
}
