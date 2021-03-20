//! Tests auto-converted from "sass-spec/spec/directives/warn/escaped.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@w\\61rn warning;\
            \na {b: c}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
