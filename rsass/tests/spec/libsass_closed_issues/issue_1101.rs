//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1101.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1101")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n$foo: white;\r\
             \nfoo {\r\
             \n  bar: color.adjust($foo, $hue: -6deg, $lightness: -16%, $saturation: -7%);\r\
             \n}"
        ),
        "foo {\
         \n  bar: rgb(84%, 84%, 84%);\
         \n}\n"
    );
}
