//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/27_escaped_double_quotes/01_inline.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: \"\\\"\";\
            \n  output: #{\"\\\"\"};\
            \n  output: \"[#{\"\\\"\"}]\";\
            \n  output: \"#{\"\\\"\"}\";\
            \n  output: \'#{\"\\\"\"}\';\
            \n  output: \"[\'#{\"\\\"\"}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \'\"\';\
        \n  output: \";\
        \n  output: \'[\"]\';\
        \n  output: \'\"\';\
        \n  output: \'\"\';\
        \n  output: \"[\'\\\"\']\";\
        \n}\
        \n"
    );
}
