//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_185"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_185/hoisting.hrx"
#[test]
#[ignore] // wrong result
fn hoisting() {
    assert_eq!(
        rsass(
            "@media only screen {\
            \n  .foo {\
            \n    content: bar;\
            \n\
            \n    @media (min-width: 1337px) {\
            \n      content: baz;\
            \n    }\
            \n\
            \n    content: foo;\
            \n  }\
            \n}\
            \n\
            \n$foo: \"(min-width: 0) and (max-width: 599px),  (min-width: 600px) and (max-width: 899px)\";\
            \n@media #{$foo} {\
            \n  $bar: \"(min-width: 0) and (max-width: 599px)\";\
            \n  @media #{$bar} {\
            \n    .foo {\
            \n      content: bar;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media only screen {\
        \n  .foo {\
        \n    content: bar;\
        \n    content: foo;\
        \n  }\
        \n}\
        \n@media only screen and (min-width: 1337px) {\
        \n  .foo {\
        \n    content: baz;\
        \n  }\
        \n}\
        \n@media (min-width: 0) and (max-width: 599px) and (min-width: 0) and (max-width: 599px), (min-width: 600px) and (max-width: 899px) and (min-width: 0) and (max-width: 599px) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_185/media_level_4.hrx"
#[test]
#[ignore] // wrong result
fn media_level_4() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  @media (pointer: none) {\
            \n    content: foo;\
            \n\
            \n    @media (scripting) {\
            \n      content: baz;\
            \n\
            \n      @media (light-level: dim) {\
            \n        content: bar;\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (pointer: none) {\
        \n  .foo {\
        \n    content: foo;\
        \n  }\
        \n}\
        \n@media (pointer: none) and (scripting) {\
        \n  .foo {\
        \n    content: baz;\
        \n  }\
        \n}\
        \n@media (pointer: none) and (scripting) and (light-level: dim) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_185/media_wrapper_selector.hrx"
#[test]
#[ignore] // wrong result
fn media_wrapper_selector() {
    assert_eq!(
        rsass(
            "@media all {\
            \n  .bar { content: baz; }\
            \n\
            \n  @media (min-width: 1337px) {\
            \n    .foo { content: bar; }\
            \n  }\
            \n}\
            \n\
            \n@media all {\
            \n  .bar { content: baz; }\
            \n\
            \n  @media (min-width: 1337px) {\
            \n    .baz { content: foo; }\
            \n\
            \n    @media (max-width: 42em) {\
            \n      .foo { content: bar; }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media all {\
        \n  .bar {\
        \n    content: baz;\
        \n  }\
        \n}\
        \n@media (min-width: 1337px) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n@media all {\
        \n  .bar {\
        \n    content: baz;\
        \n  }\
        \n}\
        \n@media (min-width: 1337px) {\
        \n  .baz {\
        \n    content: foo;\
        \n  }\
        \n}\
        \n@media (min-width: 1337px) and (max-width: 42em) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_185/merge_no_repeat.hrx"
#[test]
#[ignore] // wrong result
fn merge_no_repeat() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  content: foo;\
            \n\
            \n  @media only screen and (min-width: 1337px) {\
            \n    content: bar;\
            \n\
            \n    @media only screen and (max-width: 42em) {\
            \n      content: baz;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  content: foo;\
        \n}\
        \n@media only screen and (min-width: 1337px) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n@media only screen and (min-width: 1337px) and (max-width: 42em) {\
        \n  .foo {\
        \n    content: baz;\
        \n  }\
        \n}\
        \n"
    );
}

// Ignoring "mixin.hrx", not expected to work yet.

// From "sass-spec/spec/libsass-closed-issues/issue_185/selector_wrapper_media.hrx"
#[test]
#[ignore] // wrong result
fn selector_wrapper_media() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  @media all {\
            \n    content: baz;\
            \n\
            \n    @media (min-width: 1337px) {\
            \n      content: bar;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \n.foo {\
            \n  @media all {\
            \n    content: baz;\
            \n\
            \n    @media (min-width: 1337px) {\
            \n      content: foo;\
            \n\
            \n      @media (max-width: 42em) {\
            \n        content: bar;\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media all {\
        \n  .foo {\
        \n    content: baz;\
        \n  }\
        \n}\
        \n@media (min-width: 1337px) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n@media all {\
        \n  .foo {\
        \n    content: baz;\
        \n  }\
        \n}\
        \n@media (min-width: 1337px) {\
        \n  .foo {\
        \n    content: foo;\
        \n  }\
        \n}\
        \n@media (min-width: 1337px) and (max-width: 42em) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n"
    );
}
