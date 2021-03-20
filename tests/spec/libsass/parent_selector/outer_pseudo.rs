//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector/outer-pseudo.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo bar {\
            \n    &:baz {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo + bar {\
            \n    &:baz {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo > bar {\
            \n    &:baz {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo ~ bar {\
            \n    &:baz {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo bar:baz {\
        \n  bam: true;\
        \n}\
        \nfoo + bar:baz {\
        \n  bam: true;\
        \n}\
        \nfoo > bar:baz {\
        \n  bam: true;\
        \n}\
        \nfoo ~ bar:baz {\
        \n  bam: true;\
        \n}\
        \n"
    );
}
