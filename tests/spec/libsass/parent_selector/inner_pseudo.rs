//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector/inner-pseudo.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n    &:bar baz {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo {\
            \n    &:bar + baz {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo {\
            \n    &:bar > baz {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo {\
            \n    &:bar ~ baz {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo:bar baz {\
        \n  bam: true;\
        \n}\
        \nfoo:bar + baz {\
        \n  bam: true;\
        \n}\
        \nfoo:bar > baz {\
        \n  bam: true;\
        \n}\
        \nfoo:bar ~ baz {\
        \n  bam: true;\
        \n}\
        \n"
    );
}
