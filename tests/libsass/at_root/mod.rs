//! Tests auto-converted from "sass-spec/spec/libsass/at-root"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/libsass/at-root/135_test_simple_at_root"
#[test]
fn t135_test_simple_at_root() {
    assert_eq!(
        rsass(".foo {\n  @at-root {\n    .bar {a: b}\n  }\n}\n").unwrap(),
        ".bar {\n  a: b;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/at-root/136_test_at_root_with_selector"
#[test]
fn t136_test_at_root_with_selector() {
    assert_eq!(
        rsass(".foo {\n  @at-root .bar {a: b}\n}\n").unwrap(),
        ".bar {\n  a: b;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/at-root/137_test_at_root_in_mixin"
#[test]
fn t137_test_at_root_in_mixin() {
    assert_eq!(
        rsass(
            "@mixin bar {\n  @at-root .bar {a: b}\n}\n.foo {\n  @include bar;\n}\n"
        )
        .unwrap(),
        ".bar {\n  a: b;\n}\n"
    );
}

// Ignoring "138_test_at_root_in_media", not expected to work yet.

// Ignoring "139_test_at_root_in_bubbled_media", not expected to work yet.

// Ignoring "140_test_at_root_in_unknown_directive", not expected to work yet.

/// From "sass-spec/spec/libsass/at-root/141_test_at_root_with_parent_ref"
#[test]
fn t141_test_at_root_with_parent_ref() {
    assert_eq!(
        rsass(".foo {\n  @at-root & {\n    a: b;\n  }\n}\n").unwrap(),
        ".foo {\n  a: b;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/at-root/142_test_multi_level_at_root_with_parent_ref"
#[test]
fn t142_test_multi_level_at_root_with_parent_ref() {
    assert_eq!(
        rsass(
            ".foo {\n  @at-root & {\n    .bar {\n      @at-root & {\n        a: b;\n      }\n    }\n  }\n}\n"
        )
        .unwrap(),
        ".foo .bar {\n  a: b;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/at-root/143_test_multi_level_at_root_with_inner_parent_ref"
#[test]
fn t143_test_multi_level_at_root_with_inner_parent_ref() {
    assert_eq!(
        rsass(
            ".foo {\n  @at-root .bar {\n    @at-root & {\n      a: b;\n    }\n  }\n}\n"
        )
        .unwrap(),
        ".bar {\n  a: b;\n}\n"
    );
}

// Ignoring "ampersand", not expected to work yet.

/// From "sass-spec/spec/libsass/at-root/basic"
#[test]
fn basic() {
    assert_eq!(
        rsass(
            "foo {\n  color: blue;\n  @at-root {\n    bar {\n      color: red;\n    }\n  }\n}\nfoo {\n  color: blue;\n  @at-root bar {\n    color: red;\n  }\n}\n"
        )
        .unwrap(),
        "foo {\n  color: blue;\n}\nbar {\n  color: red;\n}\nfoo {\n  color: blue;\n}\nbar {\n  color: red;\n}\n"
    );
}

// Ignoring "extend", not expected to work yet.

/// From "sass-spec/spec/libsass/at-root/keyframes"
#[test]
fn keyframes() {
    assert_eq!(
        rsass(
            "foo {\n  color: red;\n  @at-root {\n    @keyframes animation {\n      to { color: red; }\n    }\n  }\n  bar {\n    color: blue;\n    @at-root {\n      @keyframes other-animation {\n        to { color: blue; }\n      }\n    }\n  }\n}\n"
        )
        .unwrap(),
        "foo {\n  color: red;\n}\n@keyframes animation {\n  to {\n    color: red;\n  }\n}\nfoo bar {\n  color: blue;\n}\n@keyframes other-animation {\n  to {\n    color: blue;\n  }\n}\n"
    );
}

/// From "sass-spec/spec/libsass/at-root/media"
#[test]
fn media() {
    assert_eq!(
        rsass(
            "foo {\n  @at-root {\n    @media print {\n      bar {\n        color: red;\n      }\n    }\n    baz {\n      @media speech {\n        color: blue;\n      }\n    }\n  }\n}\n"
        )
        .unwrap(),
        "@media print {\n  bar {\n    color: red;\n  }\n}\n@media speech {\n  baz {\n    color: blue;\n  }\n}\n"
    );
}

/// From "sass-spec/spec/libsass/at-root/nested"
#[test]
fn nested() {
    assert_eq!(
        rsass(
            "foo {\n  color: blue;\n  baz {\n    color: purple;\n    @at-root {\n      bar {\n        color: red;\n      }\n    }\n  }\n}\nfoo {\n  color: blue;\n  baz {\n    color: purple;\n    @at-root bar {\n      color: red;\n    }\n  }\n}\n"
        )
        .unwrap(),
        "foo {\n  color: blue;\n}\nfoo baz {\n  color: purple;\n}\nbar {\n  color: red;\n}\nfoo {\n  color: blue;\n}\nfoo baz {\n  color: purple;\n}\nbar {\n  color: red;\n}\n"
    );
}

// Ignoring "with_without", not expected to work yet.
