//! Tests auto-converted from "sass-spec/spec/css/blockless_directive_without_semicolon.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@foo \"bar\";\
            \n"
        )
        .unwrap(),
        "@foo \"bar\";\
        \n"
    );
}
