//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-prefix.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-root-prefix")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@at-root {\r\
             \n  pre& {\r\
             \n    foo {\r\
             \n      bar: baz;\r\
             \n    }\r\
             \n  }\r\
             \n}"
        ),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n2 |   pre& {\
         \n  |      ^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
