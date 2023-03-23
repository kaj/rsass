//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-alone-itpl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-alone-itpl").mock_file(
        "include.scss",
        "#{&} {\r\n  foo {\r\n    bar: baz;\r\n  }\r\n}\r\n",
    )
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err("@import \"include.scss\";"),
        "Error: expected selector.\
         \n  ,\
         \n1 | #{&} {\
         \n  |      ^\
         \n  \'\
         \n  include.scss 1:6  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
