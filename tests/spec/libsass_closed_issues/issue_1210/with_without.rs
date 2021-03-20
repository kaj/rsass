//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1210/with_without.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
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
