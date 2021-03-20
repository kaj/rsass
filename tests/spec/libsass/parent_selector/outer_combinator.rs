//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector/outer-combinator.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo bar {\
            \n    & baz {\
            \n        bam: true;\
            \n    }\
            \n    baz & {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo + bar {\
            \n    & baz {\
            \n        bam: true;\
            \n    }\
            \n    baz & {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo > bar {\
            \n    & baz {\
            \n        bam: true;\
            \n    }\
            \n    baz & {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo ~ bar {\
            \n    & baz {\
            \n        bam: true;\
            \n    }\
            \n    baz & {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo bar baz {\
        \n  bam: true;\
        \n}\
        \nbaz foo bar {\
        \n  bam: true;\
        \n}\
        \nfoo + bar baz {\
        \n  bam: true;\
        \n}\
        \nbaz foo + bar {\
        \n  bam: true;\
        \n}\
        \nfoo > bar baz {\
        \n  bam: true;\
        \n}\
        \nbaz foo > bar {\
        \n  bam: true;\
        \n}\
        \nfoo ~ bar baz {\
        \n  bam: true;\
        \n}\
        \nbaz foo ~ bar {\
        \n  bam: true;\
        \n}\
        \n"
    );
}
