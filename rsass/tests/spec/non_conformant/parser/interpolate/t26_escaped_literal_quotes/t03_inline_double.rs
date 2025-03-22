//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/26_escaped_literal_quotes/03_inline_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("03_inline_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: #{#{\\\"\\\'}};\
             \n  output: #{\"[#{\\\"\\\'}]\"};\
             \n  output: #{\"#{\\\"\\\'}\"};\
             \n  output: #{\'#{\\\"\\\'}\'};\
             \n  output: #{\"[\'#{\\\"\\\'}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: \\\"\\\';\
         \n  output: [\\\"\\\'];\
         \n  output: \\\"\\\';\
         \n  output: \\\"\\\';\
         \n  output: [\'\\\"\\\'\'];\
         \n}\n"
    );
}
