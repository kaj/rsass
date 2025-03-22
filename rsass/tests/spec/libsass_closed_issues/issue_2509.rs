//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2509.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2509")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "[charset i] {\r\
             \n\tdisplay: block;\r\
             \n}\r\
             \n\r\
             \n[charset I] {\r\
             \n\tdisplay: block;\r\
             \n}\r\
             \n\r\
             \n[charset=\"utf-8\" i] {\r\
             \n\tdisplay: block;\r\
             \n}\r\
             \n\r\
             \n[charset=\"utf-8\" I] {\r\
             \n\tdisplay: block;\r\
             \n}"
        ),
        "Error: Expected \"]\".\
         \n  ,\
         \n1 | [charset i] {\
         \n  |          ^\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
    );
}
