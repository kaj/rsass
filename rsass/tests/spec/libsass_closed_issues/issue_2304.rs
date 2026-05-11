//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2304.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("issue_2304")
        .mock_file("_module.scss", ".foo, & {\n  background: red;\n}\n\n.foo, &:before {\n  background: red;\n}")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@import \"module\";"),
        ".foo, & {\
         \n  background: red;\
         \n}\
         \n.foo, &:before {\
         \n  background: red;\
         \n}\n"
    );
}
