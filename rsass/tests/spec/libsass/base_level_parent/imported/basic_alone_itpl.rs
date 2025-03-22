//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-alone-itpl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-alone-itpl").mock_file(
        "include.scss",
        "#{&} {\r\n  foo {\r\n    bar: baz;\r\n  }\r\n}\r\n",
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
         \nError: expected selector.\
         \n  ,\
         \n1 | #{&} {\
         \n  |      ^\
         \n  \'\
         \n  include.scss 1:6  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
