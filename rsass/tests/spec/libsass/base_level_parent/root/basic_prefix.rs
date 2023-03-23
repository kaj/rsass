//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-prefix.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-prefix")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "pre& {\r\
             \n  foo {\r\
             \n    bar: baz;\r\
             \n  }\r\
             \n}"
        ),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n1 | pre& {\
         \n  |    ^\
         \n  \'\
         \n  input.scss 1:4  root stylesheet",
    );
}
