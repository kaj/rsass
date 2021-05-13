//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/01_inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
