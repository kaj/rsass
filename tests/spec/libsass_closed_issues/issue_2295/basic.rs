//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2295/basic.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic").mock_file(
        "include.scss",
        "@if (true) {\r\n  .foo { display: none; }\r\n}",
    )
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".my-scope {\r\
             \n  @import \'include.scss\';\r\
             \n}"),
        ".my-scope .foo {\
         \n  display: none;\
         \n}\n"
    );
}
