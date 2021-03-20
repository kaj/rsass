//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_550/literal/number.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "#foo {\
            \n  a: 10.0001;\
            \n  b: 10.01;\
            \n  c: -10.0001;\
            \n  d: -10.01; }\
            \n\
            \n#foo {\
            \n  a: 0.0001;\
            \n  b: 0.01;\
            \n  c: -0.0001;\
            \n  d: -0.01; }\
            \n\
            \n#foo {\
            \n  a: .0001;\
            \n  b: .01;\
            \n  c: -.0001;\
            \n  d: -.01; }\
            \n"
        )
        .unwrap(),
        "#foo {\
        \n  a: 10.0001;\
        \n  b: 10.01;\
        \n  c: -10.0001;\
        \n  d: -10.01;\
        \n}\
        \n#foo {\
        \n  a: 0.0001;\
        \n  b: 0.01;\
        \n  c: -0.0001;\
        \n  d: -0.01;\
        \n}\
        \n#foo {\
        \n  a: 0.0001;\
        \n  b: 0.01;\
        \n  c: -0.0001;\
        \n  d: -0.01;\
        \n}\
        \n"
    );
}
