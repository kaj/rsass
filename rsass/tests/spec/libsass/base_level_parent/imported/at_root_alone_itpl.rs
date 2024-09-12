//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/at-root-alone-itpl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("at-root-alone-itpl")
        .mock_file("include.scss", "@at-root {\r\n  #{&} {\r\n    foo {\r\n      bar: baz;\r\n    }\r\n  }\r\n}\r\n")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@import \"include.scss\";"),
        "Error: expected selector.\
         \n  ,\
         \n2 |   #{&} {\
         \n  |        ^\
         \n  \'\
         \n  include.scss 2:8  @import\
         \n  input.scss 1:9    root stylesheet",
    );
}
