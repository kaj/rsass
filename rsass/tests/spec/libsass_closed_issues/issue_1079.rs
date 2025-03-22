//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1079.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1079")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("#{hdr(2,5)} { color: #08c; }"),
        "Error: expected selector.\
         \n  ,--> input.scss\
         \n1 | #{hdr(2,5)} { color: #08c; }\
         \n  |   ^^^^^^^^ \
         \n  \'\
         \n  ,\
         \n1 | hdr(2, 5) \
         \n  |    = error in interpolated output\
         \n  \'\
         \n  input.scss 1:3  root stylesheet",
    );
}
