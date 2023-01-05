//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/01_literal/06_escape_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_escape_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: literal;\
             \n.result {\
             \n  output: \"[\\#{literal}]\";\
             \n  output: \"\\#{literal}\";\
             \n  output: \'\\#{literal}\';\
             \n  output: \"[\'\\#{literal}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{literal}]\";\
         \n  output: \"#{literal}\";\
         \n  output: \"#{literal}\";\
         \n  output: \"[\'#{literal}\']\";\
         \n}\n"
    );
}
