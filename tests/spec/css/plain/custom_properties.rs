//! Tests auto-converted from "sass-spec/spec/css/plain/custom_properties.hrx"

#[test]
#[ignore] // wrong result
fn arbitrary_tokens() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  --b: `~@#$%^&*()_-+={[]}|?/><;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn color() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  --b: #ff0000;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn identifier() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  --b: c;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn nested() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  --b: {c: d};\
        \n}\
        \n"
    );
}
