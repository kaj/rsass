//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-prefix.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "pre& {\r\
             \n  foo {\r\
             \n    bar: baz;\r\
             \n  }\r\
             \n}"
        )
        .unwrap_err(),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n1 | pre&{\
         \n  |    ^\
         \n  \'\
         \n  input.scss 1:4  root stylesheet\
         \n",
    );
}
