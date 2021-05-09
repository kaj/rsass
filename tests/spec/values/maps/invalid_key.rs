//! Tests auto-converted from "sass-spec/spec/values/maps/invalid-key.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("$id: inspect((a,b:c)...)\n"),
        "Error: expected \")\".\
         \n  ,\
         \n1 | $id: inspect((a,b:c)...)\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
    );
}
