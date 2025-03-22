//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_238760.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_238760")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\n\
             \n$id: meta.inspect((a#b:c)...)"
        ),
        "Error: Variable keyword argument map must have string keys.\
         \n(a #b) is not a string in (a #b: c).\
         \n  ,\
         \n3 | $id: meta.inspect((a#b:c)...)\
         \n  |                   ^^^^^^^\
         \n  \'\
         \n  input.scss 3:19  root stylesheet",
    );
}
