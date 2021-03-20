//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/07_comma_list_complex/03_inline_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: #{#{gamma, \"\'\"delta\"\'\"}};\
            \n  output: #{\"[#{gamma, \"\'\"delta\"\'\"}]\"};\
            \n  output: #{\"#{gamma, \"\'\"delta\"\'\"}\"};\
            \n  output: #{\'#{gamma, \"\'\"delta\"\'\"}\'};\
            \n  output: #{\"[\'#{gamma, \"\'\"delta\"\'\"}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: gamma, \' delta \';\
        \n  output: [gamma, \' delta \'];\
        \n  output: gamma, \' delta \';\
        \n  output: gamma, \' delta \';\
        \n  output: [\'gamma, \' delta \'\'];\
        \n}\
        \n"
    );
}
