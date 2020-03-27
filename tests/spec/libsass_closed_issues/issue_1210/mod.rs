//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1210"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1210/ampersand.hrx"
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
            \n\
            \nfoo {\
            \n  color: blue;\
            \n\
            \n  @at-root {\
            \n    & bar {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  color: blue;\
            \n\
            \n  @at-root {\
            \n    bar & {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  color: blue;\
            \n\
            \n  @at-root {\
            \n    bar {\
            \n        & baz {\
            \n            color: red;\
            \n        }\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n    @at-root bar & {\
            \n        color: red;\
            \n\
            \n        & baz {\
            \n            color: blue;\
            \n        }\
            \n    }\
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
        \nfoo {\
        \n  color: blue;\
        \n}\
        \nfoo bar {\
        \n  color: red;\
        \n}\
        \nfoo {\
        \n  color: blue;\
        \n}\
        \nbar foo {\
        \n  color: red;\
        \n}\
        \nfoo {\
        \n  color: blue;\
        \n}\
        \nbar baz {\
        \n  color: red;\
        \n}\
        \nbar foo {\
        \n  color: red;\
        \n}\
        \nbar foo baz {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1210/basic.hrx"
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
            \n\
            \nfoo {\
            \n  color: blue;\
            \n\
            \n  @at-root bar {\
            \n    baz {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  color: blue;\
            \n\
            \n  @at-root {\
            \n    bar {\
            \n      baz {\
            \n        color: red;\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n\
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
        \nfoo {\
        \n  color: blue;\
        \n}\
        \nbar baz {\
        \n  color: red;\
        \n}\
        \nfoo {\
        \n  color: blue;\
        \n}\
        \nbar baz {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1210/extend.hrx"
#[test]
#[ignore] // wrong result
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
            \n\
            \n  %ampersand-placeholder & {\
            \n    color: green;\
            \n  }\
            \n\
            \n  @at-root {\
            \n    qux {\
            \n      @extend %ampersand-placeholder;\
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
            \n\
            \nbam {\
            \n  @extend %ampersand-placeholder;\
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
        \nbam foo, qux foo {\
        \n  color: green;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1210/keyframes.hrx"
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

// From "sass-spec/spec/libsass-closed-issues/issue_1210/media.hrx"
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

// From "sass-spec/spec/libsass-closed-issues/issue_1210/nested.hrx"
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

// From "sass-spec/spec/libsass-closed-issues/issue_1210/with_without.hrx"
#[test]
#[ignore] // unexepected error
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
