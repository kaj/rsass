//! Tests auto-converted from "sass-spec/spec/libsass/at-root"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/libsass/at-root/135_test_simple_at_root.hrx"
#[test]
fn t135_test_simple_at_root() {
    assert_eq!(
        rsass(
            ".foo {\
             \n  @at-root {\
             \n    .bar {a: b}\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        ".bar {\
         \n  a: b;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/136_test_at_root_with_selector.hrx"
#[test]
fn t136_test_at_root_with_selector() {
    assert_eq!(
        rsass(
            ".foo {\
             \n  @at-root .bar {a: b}\
             \n}\
             \n"
        )
        .unwrap(),
        ".bar {\
         \n  a: b;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/137_test_at_root_in_mixin.hrx"
#[test]
fn t137_test_at_root_in_mixin() {
    assert_eq!(
        rsass(
            "@mixin bar {\
             \n  @at-root .bar {a: b}\
             \n}\
             \n\
             \n.foo {\
             \n  @include bar;\
             \n}\
             \n"
        )
        .unwrap(),
        ".bar {\
         \n  a: b;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/138_test_at_root_in_media.hrx"
#[test]
fn t138_test_at_root_in_media() {
    assert_eq!(
        rsass(
            "@media screen {\
             \n  .foo {\
             \n    @at-root .bar {a: b}\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        "@media screen {\
         \n  .bar {\
         \n    a: b;\
         \n  }\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/139_test_at_root_in_bubbled_media.hrx"
#[test]
fn t139_test_at_root_in_bubbled_media() {
    assert_eq!(
        rsass(
            ".foo {\
             \n  @media screen {\
             \n    @at-root .bar {a: b}\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        "@media screen {\
         \n  .bar {\
         \n    a: b;\
         \n  }\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/140_test_at_root_in_unknown_directive.hrx"
#[test]
fn t140_test_at_root_in_unknown_directive() {
    assert_eq!(
        rsass(
            "@fblthp {\
             \n  .foo {\
             \n    @at-root .bar {a: b}\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        "@fblthp {\
         \n  .bar {\
         \n    a: b;\
         \n  }\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/141_test_at_root_with_parent_ref.hrx"
#[test]
fn t141_test_at_root_with_parent_ref() {
    assert_eq!(
        rsass(
            ".foo {\
             \n  @at-root & {\
             \n    a: b;\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        ".foo {\
         \n  a: b;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/142_test_multi_level_at_root_with_parent_ref.hrx"
#[test]
fn t142_test_multi_level_at_root_with_parent_ref() {
    assert_eq!(
        rsass(
            ".foo {\
             \n  @at-root & {\
             \n    .bar {\
             \n      @at-root & {\
             \n        a: b;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        ".foo .bar {\
         \n  a: b;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/143_test_multi_level_at_root_with_inner_parent_ref.hrx"
#[test]
fn t143_test_multi_level_at_root_with_inner_parent_ref() {
    assert_eq!(
        rsass(
            ".foo {\
             \n  @at-root .bar {\
             \n    @at-root & {\
             \n      a: b;\
             \n    }\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        ".bar {\
         \n  a: b;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/ampersand.hrx"
#[test]
fn ampersand() {
    assert_eq!(
        rsass(
            "foo {\
             \n  @at-root {\
             \n    & {\
             \n      color: blue;\
             \n    }\
             \n\
             \n    &--modifier {\
             \n      color: red;\
             \n    }\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        "foo {\
         \n  color: blue;\
         \n}\
         \nfoo--modifier {\
         \n  color: red;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/basic.hrx"
#[test]
fn basic() {
    assert_eq!(
        rsass(
            "foo {\
             \n  color: blue;\
             \n\
             \n  @at-root {\
             \n    bar {\
             \n      color: red;\
             \n    }\
             \n  }\
             \n}\
             \n\
             \nfoo {\
             \n  color: blue;\
             \n\
             \n  @at-root bar {\
             \n    color: red;\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        "foo {\
         \n  color: blue;\
         \n}\
         \nbar {\
         \n  color: red;\
         \n}\
         \nfoo {\
         \n  color: blue;\
         \n}\
         \nbar {\
         \n  color: red;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/extend.hrx"
#[test]
#[ignore] // failing
fn extend() {
    assert_eq!(
        rsass(
            "foo {\
             \n  @at-root {\
             \n    %placeholder {\
             \n      color: red;\
             \n    }\
             \n  }\
             \n\
             \n  baz {\
             \n    @at-root {\
             \n      %other-placeholder {\
             \n        color: blue;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \nbar {\
             \n  @extend %placeholder;\
             \n}\
             \n\
             \nbaz {\
             \n  @extend %other-placeholder;\
             \n}\
             \n"
        )
        .unwrap(),
        "bar {\
         \n  color: red;\
         \n}\
         \nbaz {\
         \n  color: blue;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/keyframes.hrx"
#[test]
fn keyframes() {
    assert_eq!(
        rsass(
            "foo {\
             \n  color: red;\
             \n\
             \n  @at-root {\
             \n    @keyframes animation {\
             \n      to { color: red; }\
             \n    }\
             \n  }\
             \n\
             \n  bar {\
             \n    color: blue;\
             \n\
             \n    @at-root {\
             \n      @keyframes other-animation {\
             \n        to { color: blue; }\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        "foo {\
         \n  color: red;\
         \n}\
         \n@keyframes animation {\
         \n  to {\
         \n    color: red;\
         \n  }\
         \n}\
         \nfoo bar {\
         \n  color: blue;\
         \n}\
         \n@keyframes other-animation {\
         \n  to {\
         \n    color: blue;\
         \n  }\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/media.hrx"
#[test]
fn media() {
    assert_eq!(
        rsass(
            "foo {\
             \n  @at-root {\
             \n    @media print {\
             \n      bar {\
             \n        color: red;\
             \n      }\
             \n    }\
             \n\
             \n    baz {\
             \n      @media speech {\
             \n        color: blue;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        "@media print {\
         \n  bar {\
         \n    color: red;\
         \n  }\
         \n}\
         \n@media speech {\
         \n  baz {\
         \n    color: blue;\
         \n  }\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/nested.hrx"
#[test]
fn nested() {
    assert_eq!(
        rsass(
            "foo {\
             \n  color: blue;\
             \n\
             \n  baz {\
             \n    color: purple;\
             \n\
             \n    @at-root {\
             \n      bar {\
             \n        color: red;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \nfoo {\
             \n  color: blue;\
             \n\
             \n  baz {\
             \n    color: purple;\
             \n\
             \n    @at-root bar {\
             \n      color: red;\
             \n    }\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        "foo {\
         \n  color: blue;\
         \n}\
         \nfoo baz {\
         \n  color: purple;\
         \n}\
         \nbar {\
         \n  color: red;\
         \n}\
         \nfoo {\
         \n  color: blue;\
         \n}\
         \nfoo baz {\
         \n  color: purple;\
         \n}\
         \nbar {\
         \n  color: red;\
         \n}\
         \n"
    );
}

// From "sass-spec/spec/libsass/at-root/with_without.hrx"
#[test]
#[ignore] // failing
fn with_without() {
    assert_eq!(
        rsass(
            "// Unquoted\
             \n\
             \n@media (min-width: 1337px) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\
             \n\
             \n  @at-root (without: media) {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\
             \n\
             \n  @at-root (without: all) {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@supports (color: red) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\
             \n\
             \n  @at-root (without: supports) {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@supports (color: red) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\
             \n\
             \n  @at-root (without: all) {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: all) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: media supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: media) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: all) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: media supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: media) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n// Quoted\
             \n\
             \n@media (min-width: 1337px) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\
             \n\
             \n  @at-root (without: \"media\") {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\
             \n\
             \n  @at-root (without: \"all\") {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@supports (color: red) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\
             \n\
             \n  @at-root (without: \"supports\") {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@supports (color: red) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\
             \n\
             \n  @at-root (without: \"all\") {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: \"all\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: \"media\" \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: \"media\" supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: media \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: \"media\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: \"all\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: \"media\" \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: \"media\" supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: media \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: \"media\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\
             \n"
        )
        .unwrap(),
        "@media (min-width: 1337px) {\
         \n  .foo {\
         \n    content: baz;\
         \n  }\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  .foo {\
         \n    content: baz;\
         \n  }\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n@supports (color: red) {\
         \n  .foo {\
         \n    content: baz;\
         \n  }\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n@supports (color: red) {\
         \n  .foo {\
         \n    content: baz;\
         \n  }\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  @supports (color: red) {\
         \n    .foo {\
         \n      content: bar;\
         \n    }\
         \n  }\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n@supports (color: red) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  @supports (color: red) {\
         \n    .foo {\
         \n      content: bar;\
         \n    }\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  @supports (color: red) {\
         \n    .foo {\
         \n      content: bar;\
         \n    }\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  @supports (color: red) {\
         \n    .foo {\
         \n      content: bar;\
         \n    }\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\
         \n@supports (color: red) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  .foo {\
         \n    content: baz;\
         \n  }\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  .foo {\
         \n    content: baz;\
         \n  }\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n@supports (color: red) {\
         \n  .foo {\
         \n    content: baz;\
         \n  }\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n@supports (color: red) {\
         \n  .foo {\
         \n    content: baz;\
         \n  }\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n.foo {\
         \n  content: bar;\
         \n}\
         \n@supports (color: red) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  @supports (color: red) {\
         \n    .foo {\
         \n      content: bar;\
         \n    }\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  @supports (color: red) {\
         \n    .foo {\
         \n      content: bar;\
         \n    }\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  @supports (color: red) {\
         \n    .foo {\
         \n      content: bar;\
         \n    }\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  @supports (color: red) {\
         \n    .foo {\
         \n      content: bar;\
         \n    }\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\
         \n@supports (color: red) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\
         \n"
    );
}
