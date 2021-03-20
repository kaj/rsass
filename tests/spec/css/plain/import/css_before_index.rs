//! Tests auto-converted from "sass-spec/spec/css/plain/import/css_before_index.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \'other\';\
            \n"
        )
        .unwrap(),
        "other {\
        \n  index: false;\
        \n}\
        \n"
    );
}
