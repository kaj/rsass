//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/does_not_move_page_block_in_media.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("does_not_move_page_block_in_media")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media screen {\
             \n  a { x:y; }\
             \n  @page {}\
             \n}"),
        "@media screen {\
         \n  a {\
         \n    x: y;\
         \n  }\
         \n  @page {}\
         \n}\n"
    );
}
