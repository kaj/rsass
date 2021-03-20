//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/171_test_loud_comment_in_compressed_mode.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "/*! foo */\
            \n"
        )
        .unwrap(),
        "/*! foo */\
        \n"
    );
}
