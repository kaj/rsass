//! Tests auto-converted from "sass-spec/spec/css/function_name_identifiers.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: url;\
            \n  c: calc;\
            \n  d: element;\
            \n  e: expression;\
            \n  f: progid;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: url;\
        \n  c: calc;\
        \n  d: element;\
        \n  e: expression;\
        \n  f: progid;\
        \n}\
        \n"
    );
}
