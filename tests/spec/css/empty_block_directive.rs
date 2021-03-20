//! Tests auto-converted from "sass-spec/spec/css/empty_block_directive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@foo {}\
            \n"
        )
        .unwrap(),
        "@foo {}\
        \n"
    );
}
