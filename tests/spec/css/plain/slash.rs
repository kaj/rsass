//! Tests auto-converted from "sass-spec/spec/css/plain/slash.hrx"

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
        \n  slash: 1/2/foo/bar;\
        \n}\
        \n"
    );
}
