//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1889.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@media (min-width: 640px) { \
            \n  /* comment */\
            \n}\
            \n\
            \ndiv {\
            \n  @media (min-width: 320px) { \
            \n    /* comment */\
            \n  }\
            \n}"
        )
        .unwrap(),
        "@media (min-width: 640px) {\
        \n  /* comment */\
        \n}\
        \n@media (min-width: 320px) {\
        \n  div {\
        \n    /* comment */\
        \n  }\
        \n}\
        \n"
    );
}
