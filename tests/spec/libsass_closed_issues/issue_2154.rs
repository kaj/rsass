//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2154.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@media (min-width: 1px) {\
            \n  .first {\
            \n    font-weight: 100;\
            \n\
            \n    @media (min-width: 2px) {}\
            \n  }\
            \n  .second {\
            \n    font-weight: 200;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (min-width: 1px) {\
        \n  .first {\
        \n    font-weight: 100;\
        \n  }\
        \n  .second {\
        \n    font-weight: 200;\
        \n  }\
        \n}\
        \n"
    );
}
