//! Tests auto-converted from "sass-spec/spec/libsass/bool.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  a: (false and \"hey\");\
            \n  b: (\"hey\" and \"ho\");\
            \n  b: (\"hey\" or \"ho\");\
            \n  a: false and \"hey\";\
            \n  b: \"hey\" and \"ho\";\
            \n  b: unquote(\"hey\") or \"ho\";\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: false;\
        \n  b: \"ho\";\
        \n  b: \"hey\";\
        \n  a: false;\
        \n  b: \"ho\";\
        \n  b: hey;\
        \n}\
        \n"
    );
}
