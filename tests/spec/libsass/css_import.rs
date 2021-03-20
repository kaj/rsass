//! Tests auto-converted from "sass-spec/spec/libsass/css-import.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \'foo.css\', \"bar.css\";\
            \n\
            \ndiv {\
            \n  color: red;\
            \n}"
        )
        .unwrap(),
        "@import \'foo.css\';\
        \n@import \"bar.css\";\
        \ndiv {\
        \n  color: red;\
        \n}\
        \n"
    );
}
