//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2465.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  a: 4e2px;\
            \n  b: 5e-2px;\
            \n  c: 6e2px + 3px;\
            \n  d: 7e-2px + 3px;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 400px;\
        \n  b: 0.05px;\
        \n  c: 603px;\
        \n  d: 3.07px;\
        \n}\
        \n"
    );
}
