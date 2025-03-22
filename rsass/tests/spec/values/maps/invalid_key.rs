//! Tests auto-converted from "sass-spec/spec/values/maps/invalid-key.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("invalid-key")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\n\
             \n$id: meta.inspect((a,b:c)...)\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n3 | $id: meta.inspect((a,b:c)...)\
         \n  |                       ^\
         \n  \'\
         \n  input.scss 3:23  root stylesheet",
    );
}
