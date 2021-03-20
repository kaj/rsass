//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_556.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$test: (\
            \n  one: 1,\
            \n  two: 2,\
            \n);\
            \n\
            \n$expect: (\
            \n  two: 2,\
            \n  one: 1,\
            \n);\
            \n\
            \n.test {\
            \n  equal: $test == $expect;\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  equal: true;\
        \n}\
        \n"
    );
}
