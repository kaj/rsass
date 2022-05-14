//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1328.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1328")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("#{bar},\
             \n[foo=\"#{bar}\"],\
             \n[foo=\"#{bar}\"] {\
             \n    content: \"foo\";\
             \n}\n"),
        "bar,\
         \n[foo=bar],\
         \n[foo=bar] {\
         \n  content: \"foo\";\
         \n}\n"
    );
}
