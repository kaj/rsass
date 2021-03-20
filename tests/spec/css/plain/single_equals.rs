//! Tests auto-converted from "sass-spec/spec/css/plain/single_equals.hrx"

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
        \n  single-equals: alpha(opacity=65);\
        \n}\
        \n"
    );
}
