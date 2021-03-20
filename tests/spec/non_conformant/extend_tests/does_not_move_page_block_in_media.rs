//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/does_not_move_page_block_in_media.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@media screen {\
            \n  a { x:y; }\
            \n  @page {}\
            \n}"
        )
        .unwrap(),
        "@media screen {\
        \n  a {\
        \n    x: y;\
        \n  }\
        \n  @page {}\
        \n}\
        \n"
    );
}
