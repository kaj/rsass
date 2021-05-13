//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1079.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("#{hdr(2,5)} { color: #08c; }"),
        "Error: expected selector.\
         \n  ,\
         \n1 | hdr(2, 5){ color: #08c; }\
         \n  |    ^\
         \n  \'\
         \n  input.scss 1:4  root stylesheet",
    );
}
