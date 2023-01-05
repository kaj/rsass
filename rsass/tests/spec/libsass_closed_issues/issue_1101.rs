//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1101.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1101")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$foo: white;\r\
             \nfoo {\r\
             \n  bar: adjust-color($foo, $hue: -6deg, $lightness: -16%, $saturation: -7%);\r\
             \n}"
        ),
        "foo {\
         \n  bar: #d6d6d6;\
         \n}\n"
    );
}
