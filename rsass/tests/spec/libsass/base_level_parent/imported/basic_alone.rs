//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-alone.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("basic-alone")
        .mock_file("include.scss", "& {\n  foo {\n    bar: baz;\n  }\n}\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@import \"include.scss\";"
        ),
        "DEPRECATION WARNING: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"include.scss\";\
         \n  |         ^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: Top-level selectors may not contain the parent selector \"&\".\
         \n  ,\
         \n1 | & {\
         \n  | ^\
         \n  \'\
         \n  include.scss 1:1  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
