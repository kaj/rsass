//! Tests auto-converted from "sass-spec/spec/css/plain/import/in_css.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "@import \"whatever\";\
        \n"
    );
}
