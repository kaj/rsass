//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_943.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%dog {\
            \n    @media (min-width: 10px) {\
            \n        &:hover {\
            \n            display: none;\
            \n        }\
            \n    }\
            \n}\
            \n\
            \n.puppy {\
            \n    @extend %dog;\
            \n    background-color: red;\
            \n}"
        )
        .unwrap(),
        "@media (min-width: 10px) {\
        \n  .puppy:hover {\
        \n    display: none;\
        \n  }\
        \n}\
        \n.puppy {\
        \n  background-color: red;\
        \n}\
        \n"
    );
}
