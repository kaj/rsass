//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/error/color.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("color")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\n\
             \n$map: (\
             \n  red: \'foo\',\
             \n  red: \'bar\',\
             \n);\n\
             \n.foo {\
             \n  content: meta.inspect($map);\
             \n}"
        ),
        "Error: Duplicate key.\
         \n  ,\
         \n4 |   red: \'foo\',\
         \n  |   === first key\
         \n5 |   red: \'bar\',\
         \n  |   ^^^ second key\
         \n  \'\
         \n  input.scss 5:3  root stylesheet",
    );
}
