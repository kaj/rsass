//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-prefix-itpl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-prefix-itpl").mock_file(
        "include.scss",
        "pre#{&} {\r\n  foo {\r\n    bar: baz;\r\n  }\r\n}\r\n",
    )
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"include.scss\";"),
        "pre foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
