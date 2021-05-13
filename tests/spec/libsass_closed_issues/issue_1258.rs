//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1258.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "$list:   \'(-webkit-min-device-pixel-ratio: 2)\', \'(min-resolution: 192dpi)\';\
             \n$string: \'(-webkit-min-device-pixel-ratio: 2),   (min-resolution: 192dpi)\';\n\
             \n.foo {\
             \n  // I should not unquote a list, I know. But still.\
             \n  content: unquote($list);\
             \n  content: unquote($string);\
             \n}"
        ),
        ".foo {\
         \n  content: \"(-webkit-min-device-pixel-ratio: 2)\", \"(min-resolution: 192dpi)\";\
         \n  content: (-webkit-min-device-pixel-ratio: 2),   (min-resolution: 192dpi);\
         \n}\n"
    );
}
