//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/error/functioncall.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("functioncall")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\n\
             \n@function fncall($void) {\
             \n  @return \"key\";\
             \n}\n\
             \n$map: (\
             \n  fncall(1+2): \'foo\',\
             \n  fncall(1+2): \'bar\',\
             \n);\n\
             \n.foo {\
             \n  content: meta.inspect($map);\
             \n}"
        ),
        "Error: Duplicate key.\
         \n  ,\
         \n8 |   fncall(1+2): \'foo\',\
         \n  |   =========== first key\
         \n9 |   fncall(1+2): \'bar\',\
         \n  |   ^^^^^^^^^^^ second key\
         \n  \'\
         \n  input.scss 9:3  root stylesheet",
    );
}
