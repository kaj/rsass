//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector/inner-combinator.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n    & bar baz {\
            \n        bam: true;\
            \n    }\
            \n    bar baz & {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo {\
            \n    & bar + baz {\
            \n        bam: true;\
            \n    }\
            \n    bar + baz & {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo {\
            \n    & bar > baz {\
            \n        bam: true;\
            \n    }\
            \n    bar > baz & {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo {\
            \n    & bar ~ baz {\
            \n        bam: true;\
            \n    }\
            \n    bar ~ baz & {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo bar baz {\
        \n  bam: true;\
        \n}\
        \nbar baz foo {\
        \n  bam: true;\
        \n}\
        \nfoo bar + baz {\
        \n  bam: true;\
        \n}\
        \nbar + baz foo {\
        \n  bam: true;\
        \n}\
        \nfoo bar > baz {\
        \n  bam: true;\
        \n}\
        \nbar > baz foo {\
        \n  bam: true;\
        \n}\
        \nfoo bar ~ baz {\
        \n  bam: true;\
        \n}\
        \nbar ~ baz foo {\
        \n  bam: true;\
        \n}\
        \n"
    );
}
