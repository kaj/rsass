//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/at-root-prefix.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-root-prefix")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "test{\r\
             \n  @at-root {\r\
             \n    pre& {\r\
             \n      foo {\r\
             \n        bar: baz;\r\
             \n      }\r\
             \n    }\r\
             \n  }\r\
             \n}"
        ),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n3 |     pre&{\
         \n  |        ^\
         \n  \'\
         \n  input.scss 3:8  root stylesheet",
    );
}
