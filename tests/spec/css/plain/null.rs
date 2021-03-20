//! Tests auto-converted from "sass-spec/spec/css/plain/null.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  x: null;\
        \n}\
        \n"
    );
}
