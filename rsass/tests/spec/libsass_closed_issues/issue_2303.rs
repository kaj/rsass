//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2303.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("issue_2303")
        .mock_file("_module.scss", ".okay {\r\n  background: green;\r\n}\r\n\r\n@if true {\r\n  .broken {\r\n    background: red;\r\n  }\r\n}")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".wrapper-class {\r\
             \n  @import \'module\';\r\
             \n}"),
        ".wrapper-class .okay {\
         \n  background: green;\
         \n}\
         \n.wrapper-class .broken {\
         \n  background: red;\
         \n}\n"
    );
}
