//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/property.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("property")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "foo {\
             \n  bar: #{};\
             \n}\n"
        ),
        "Error: Expected expression.\
         \n  ,\
         \n2 |   bar: #{};\
         \n  |        ^^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
