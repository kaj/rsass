//! Tests auto-converted from "sass-spec/spec/css/directive_with_lots_of_whitespace.hrx"

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
