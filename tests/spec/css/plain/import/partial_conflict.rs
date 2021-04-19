//! Tests auto-converted from "sass-spec/spec/css/plain/import/partial_conflict.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
             \n"
        )
        .unwrap_err(),
        "Error: It\'s not clear which file to import. Found:\
         \n  _plain.css\
         \n  plain.css\
         \n  ,\
         \n1 | @import \"plain\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
    );
}
