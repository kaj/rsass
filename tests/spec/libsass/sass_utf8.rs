//! Tests auto-converted from "sass-spec/spec/libsass/Sáss-UŢF8.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "span.utf8-in-path {\
            \n  margin: auto;\
            \n}\
            \n"
        )
        .unwrap(),
        "span.utf8-in-path {\
        \n  margin: auto;\
        \n}\
        \n"
    );
}
