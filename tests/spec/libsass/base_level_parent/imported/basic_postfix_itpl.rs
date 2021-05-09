//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-postfix-itpl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file(
        "include.scss",
        "#{&}post {\r\n  foo {\r\n    bar: baz;\r\n  }\r\n}\r\n",
    )
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"include.scss\";"),
        "post foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
