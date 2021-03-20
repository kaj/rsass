//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/26_escaped_literal_quotes/03_inline_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: #{#{\\\"\\\'}};\
            \n  output: #{\"[#{\\\"\\\'}]\"};\
            \n  output: #{\"#{\\\"\\\'}\"};\
            \n  output: #{\'#{\\\"\\\'}\'};\
            \n  output: #{\"[\'#{\\\"\\\'}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \\\"\\\';\
        \n  output: [\\\"\\\'];\
        \n  output: \\\"\\\';\
        \n  output: \\\"\\\';\
        \n  output: [\'\\\"\\\'\'];\
        \n}\
        \n"
    );
}
