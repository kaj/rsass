//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/26_escaped_literal_quotes/06_escape_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_escape_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \\\"\\\';\
             \n.result {\
             \n  output: \"[\\#{\\\"\\\'}]\";\
             \n  output: \"\\#{\\\"\\\'}\";\
             \n  output: \'\\#{\\\"\\\'}\';\
             \n  output: \"[\'\\#{\\\"\\\'}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{\\\"\'}]\";\
         \n  output: \"#{\\\"\'}\";\
         \n  output: \"#{\\\"\'}\";\
         \n  output: \"[\'#{\\\"\'}\']\";\
         \n}\n"
    );
}
