//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1079.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass("#{hdr(2,5)} { color: #08c; }").unwrap_err(),
        "Error: expected selector.\
         \n  ,\
         \n1 | hdr(2, 5){ color: #08c; }\
         \n  |    ^\
         \n  \'\
         \n  input.scss 1:4  root stylesheet\
         \n",
    );
}
