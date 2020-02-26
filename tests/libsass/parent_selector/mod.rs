//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass/parent-selector/basic.hrx"
#[test]
fn basic() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/libsass/parent-selector/inner-combinator.hrx"
#[test]
fn inner_combinator() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/libsass/parent-selector/inner-pseudo.hrx"
#[test]
fn inner_pseudo() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/libsass/parent-selector/missing.hrx"

// Ignoring "missing", error tests are not supported yet.

// From "sass-spec/spec/libsass/parent-selector/outer-combinator.hrx"
#[test]
fn outer_combinator() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/libsass/parent-selector/outer-pseudo.hrx"
#[test]
fn outer_pseudo() {
    assert_eq!(
        rsass(
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
