//! Tests auto-converted from "sass-spec/spec/directives/import/error/top_level_declaration.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("top_level_declaration")
        .mock_file(
            "include/_upstream.scss",
            "@mixin a { b: c }\n@include a;\n",
        )
        .mock_file("parent_selector/_upstream.scss", "&{ a: b; }\n")
        .mock_file("root/_upstream.scss", "a: b;\n")
}

#[test]
#[ignore] // wrong error
fn include() {
    let runner = runner().with_cwd("include");
    assert_eq!(
        runner.err(
            "\
             \n@import \'upstream\';\n\n"
        ),
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n2 | @import \'upstream\';\
         \n  |         ^^^^^^^^^^\
         \n  \'\
         \n    input.scss 2:9  root stylesheet\n\
         \nError: Declarations may only be used within style rules.\
         \n  ,\
         \n1 | @mixin a { b: c }\
         \n  |            ^^^^^\
         \n  \'\
         \n  _upstream.scss 1:12  a()\
         \n  _upstream.scss 2:1   @import\
         \n  input.scss 2:9       root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn parent_selector() {
    let runner = runner().with_cwd("parent_selector");
    assert_eq!(
        runner.err(
            "@import \'upstream\';\n"
        ),
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \'upstream\';\
         \n  |         ^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | &{ a: b; }\
         \n  | ^\
         \n  \'\
         \n  _upstream.scss 1:1  @import\
         \n  input.scss 1:9      root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn root() {
    let runner = runner().with_cwd("root");
    assert_eq!(
        runner.err(
            "@import \'upstream\';\n"
        ),
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \'upstream\';\
         \n  |         ^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: expected \"{\".\
         \n  ,\
         \n1 | a: b;\
         \n  |     ^\
         \n  \'\
         \n  _upstream.scss 1:5  @import\
         \n  input.scss 1:9      root stylesheet",
    );
}
