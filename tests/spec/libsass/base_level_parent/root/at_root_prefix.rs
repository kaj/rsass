//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-prefix.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@at-root {\r\
             \n  pre& {\r\
             \n    foo {\r\
             \n      bar: baz;\r\
             \n    }\r\
             \n  }\r\
             \n}"
        )
        .unwrap_err(),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n2 |   pre&{\
         \n  |      ^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
    );
}
