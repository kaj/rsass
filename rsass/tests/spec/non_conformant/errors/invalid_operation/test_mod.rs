//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-operation/mod.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mod")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "test {\r\
             \n  err: 2px % red;\r\
             \n}"
        ),
        "Error: Undefined operation \"2px % red\".\
         \n  ,\
         \n2 |   err: 2px % red;\
         \n  |        ^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
