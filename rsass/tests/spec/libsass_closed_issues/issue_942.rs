//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_942.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_942")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$v: \".foo \\\
             \n.bar\";\n\
             \n#{$v} {\
             \n\tcolor: #F00;\
             \n}\n\
             \ndiv {\
             \n\tcontent: \"foo\\\
             \nbar\";\
             \n}"),
        ".foo .bar {\
         \n  color: #F00;\
         \n}\
         \ndiv {\
         \n  content: \"foobar\";\
         \n}\n"
    );
}
