//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/libsass/parent-selector/basic"
#[test]
fn basic() {
    assert_eq!(
        rsass(
            "foo bar {\n    baz & {\n        bam: true;\n    }\n}\n\nfoo {\n    bar baz & {\n        bam: true;\n    }\n}\n"
        )
        .unwrap(),
        "baz foo bar {\n  bam: true;\n}\n\nbar baz foo {\n  bam: true;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/parent-selector/inner-combinator"
#[test]
fn inner_combinator() {
    assert_eq!(
        rsass(
            "foo {\n    & bar baz {\n        bam: true;\n    }\n    bar baz & {\n        bam: true;\n    }\n}\n\nfoo {\n    & bar + baz {\n        bam: true;\n    }\n    bar + baz & {\n        bam: true;\n    }\n}\n\nfoo {\n    & bar > baz {\n        bam: true;\n    }\n    bar > baz & {\n        bam: true;\n    }\n}\n\nfoo {\n    & bar ~ baz {\n        bam: true;\n    }\n    bar ~ baz & {\n        bam: true;\n    }\n}\n"
        )
        .unwrap(),
        "foo bar baz {\n  bam: true;\n}\nbar baz foo {\n  bam: true;\n}\n\nfoo bar + baz {\n  bam: true;\n}\nbar + baz foo {\n  bam: true;\n}\n\nfoo bar > baz {\n  bam: true;\n}\nbar > baz foo {\n  bam: true;\n}\n\nfoo bar ~ baz {\n  bam: true;\n}\nbar ~ baz foo {\n  bam: true;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/parent-selector/inner-pseudo"
#[test]
fn inner_pseudo() {
    assert_eq!(
        rsass(
            "foo {\n    &:bar baz {\n        bam: true;\n    }\n}\n\nfoo {\n    &:bar + baz {\n        bam: true;\n    }\n}\n\nfoo {\n    &:bar > baz {\n        bam: true;\n    }\n}\n\nfoo {\n    &:bar ~ baz {\n        bam: true;\n    }\n}\n"
        )
        .unwrap(),
        "foo:bar baz {\n  bam: true;\n}\n\nfoo:bar + baz {\n  bam: true;\n}\n\nfoo:bar > baz {\n  bam: true;\n}\n\nfoo:bar ~ baz {\n  bam: true;\n}\n"
    );
}

// Ignoring "missing", tests with expected error not implemented yet.

/// From "sass-spec/spec/libsass/parent-selector/outer-combinator"
#[test]
fn outer_combinator() {
    assert_eq!(
        rsass(
            "foo bar {\n    & baz {\n        bam: true;\n    }\n    baz & {\n        bam: true;\n    }\n}\n\nfoo + bar {\n    & baz {\n        bam: true;\n    }\n    baz & {\n        bam: true;\n    }\n}\n\nfoo > bar {\n    & baz {\n        bam: true;\n    }\n    baz & {\n        bam: true;\n    }\n}\n\nfoo ~ bar {\n    & baz {\n        bam: true;\n    }\n    baz & {\n        bam: true;\n    }\n}\n"
        )
        .unwrap(),
        "foo bar baz {\n  bam: true;\n}\nbaz foo bar {\n  bam: true;\n}\n\nfoo + bar baz {\n  bam: true;\n}\nbaz foo + bar {\n  bam: true;\n}\n\nfoo > bar baz {\n  bam: true;\n}\nbaz foo > bar {\n  bam: true;\n}\n\nfoo ~ bar baz {\n  bam: true;\n}\nbaz foo ~ bar {\n  bam: true;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/parent-selector/outer-pseudo"
#[test]
fn outer_pseudo() {
    assert_eq!(
        rsass(
            "foo bar {\n    &:baz {\n        bam: true;\n    }\n}\n\nfoo + bar {\n    &:baz {\n        bam: true;\n    }\n}\n\nfoo > bar {\n    &:baz {\n        bam: true;\n    }\n}\n\nfoo ~ bar {\n    &:baz {\n        bam: true;\n    }\n}\n"
        )
        .unwrap(),
        "foo bar:baz {\n  bam: true;\n}\n\nfoo + bar:baz {\n  bam: true;\n}\n\nfoo > bar:baz {\n  bam: true;\n}\n\nfoo ~ bar:baz {\n  bam: true;\n}\n"
    );
}
