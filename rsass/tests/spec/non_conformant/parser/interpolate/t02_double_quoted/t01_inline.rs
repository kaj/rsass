//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/01_inline.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: \"dquoted\";\
             \n  output: #{\"dquoted\"};\
             \n  output: \"[#{\"dquoted\"}]\";\
             \n  output: \"#{\"dquoted\"}\";\
             \n  output: \'#{\"dquoted\"}\';\
             \n  output: \"[\'#{\"dquoted\"}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"dquoted\";\
         \n  output: dquoted;\
         \n  output: \"[dquoted]\";\
         \n  output: \"dquoted\";\
         \n  output: \"dquoted\";\
         \n  output: \"[\'dquoted\']\";\
         \n}\n"
    );
}
