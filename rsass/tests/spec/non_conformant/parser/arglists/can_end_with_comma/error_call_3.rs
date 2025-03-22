//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-call-3.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error-call-3")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "// incorectly terminated\
             \n.error {\
             \n  a: incorrectly-terminated($a,$b,;\
             \n}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n3 |   a: incorrectly-terminated($a,$b,;\
         \n  |                                   ^\
         \n  \'\
         \n  input.scss 3:35  root stylesheet",
    );
}
