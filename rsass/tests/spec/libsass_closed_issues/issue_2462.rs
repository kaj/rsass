//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2462.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2462")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \nb {\
             \n    color: color.adjust(Crimson, $lightness: 10%);\
             \n}\n"),
        "b {\
         \n  color: rgb(92.8431372549%, 21.2745098039%, 35.5882352941%);\
         \n}\n"
    );
}
