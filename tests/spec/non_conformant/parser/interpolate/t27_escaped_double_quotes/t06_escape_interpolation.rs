//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/27_escaped_double_quotes/06_escape_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"\\\"\";\
             \n.result {\
             \n  output: \"[\\#{\"\\\"\"}]\";\
             \n  output: \"\\#{\"\\\"\"}\";\
             \n  output: \'\\#{\"\\\"\"}\';\
             \n  output: \"[\'\\#{\"\\\"\"}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{\" \\\" \"}]\";\
         \n  output: \"#{\" \\\" \"}\";\
         \n  output: \'#{\"\"\"}\';\
         \n  output: \"[\'#{\" \\\" \"}\']\";\
         \n}\n"
    );
}
