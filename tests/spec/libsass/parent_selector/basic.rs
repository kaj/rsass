//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector/basic.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo bar {\
            \n    baz & {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n\
            \nfoo {\
            \n    bar baz & {\
            \n        bam: true;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "baz foo bar {\
        \n  bam: true;\
        \n}\
        \nbar baz foo {\
        \n  bam: true;\
        \n}\
        \n"
    );
}
