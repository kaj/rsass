//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-prefix.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-prefix").mock_file(
        "include.scss",
        "pre& {\r\n  foo {\r\n    bar: baz;\r\n  }\r\n}",
    )
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@import \"include.scss\";"
        ),
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"include.scss\";\
         \n  |         ^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n1 | pre& {\
         \n  |    ^\
         \n  \'\
         \n  include.scss 1:4  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
