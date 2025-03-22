//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/06_escape_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_escape_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"dquoted\";\
             \n.result {\
             \n  output: \"[\\#{\"dquoted\"}]\";\
             \n  output: \"\\#{\"dquoted\"}\";\
             \n  output: \'\\#{\"dquoted\"}\';\
             \n  output: \"[\'\\#{\"dquoted\"}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{\" dquoted \"}]\";\
         \n  output: \"#{\" dquoted \"}\";\
         \n  output: \'#{\"dquoted\"}\';\
         \n  output: \"[\'#{\" dquoted \"}\']\";\
         \n}\n"
    );
}
