//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1169/error/interpolate.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("interpolate")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\n\
             \n$map: (\
             \n  \'red\': \'bar\',\
             \n  #{re}#{\'d\'}: \'baz\',\
             \n);\n\
             \n.foo {\
             \n  content: meta.inspect($map);\
             \n}"
        ),
        "Error: Duplicate key.\
         \n  ,\
         \n4 |   \'red\': \'bar\',\
         \n  |   ===== first key\
         \n5 |   #{re}#{\'d\'}: \'baz\',\
         \n  |   ^^^^^^^^^^^ second key\
         \n  \'\
         \n  input.scss 5:3  root stylesheet",
    );
}
