//! Tests auto-converted from "sass-spec/spec/libsass/at-root/with_without.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("with_without")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("// Unquoted\n\
             \n@media (min-width: 1337px) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\n\
             \n  @at-root (without: media) {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\n\
             \n  @at-root (without: all) {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\n\
             \n@supports (color: red) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\n\
             \n  @at-root (without: supports) {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\n\
             \n@supports (color: red) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\n\
             \n  @at-root (without: all) {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: all) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: media supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: media) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: all) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: media supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: media) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n// Quoted\n\
             \n@media (min-width: 1337px) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\n\
             \n  @at-root (without: \"media\") {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\n\
             \n  @at-root (without: \"all\") {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\n\
             \n@supports (color: red) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\n\
             \n  @at-root (without: \"supports\") {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\n\
             \n@supports (color: red) {\
             \n  .foo {\
             \n    content: baz;\
             \n  }\n\
             \n  @at-root (without: \"all\") {\
             \n    .foo {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: \"all\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: \"media\" \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: \"media\" supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: media \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: \"media\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (without: \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: \"all\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: \"media\" \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: \"media\" supports) {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: media \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: \"media\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@media (min-width: 1337px) {\
             \n  @supports (color: red) {\
             \n    @at-root (with: \"supports\") {\
             \n      .foo {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n"),
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
         \n}\n"
    );
}
