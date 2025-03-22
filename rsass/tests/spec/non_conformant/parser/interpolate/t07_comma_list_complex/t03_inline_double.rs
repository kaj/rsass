//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/07_comma_list_complex/03_inline_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("03_inline_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: #{#{gamma, \"\'\"delta\"\'\"}};\
             \n  output: #{\"[#{gamma, \"\'\"delta\"\'\"}]\"};\
             \n  output: #{\"#{gamma, \"\'\"delta\"\'\"}\"};\
             \n  output: #{\'#{gamma, \"\'\"delta\"\'\"}\'};\
             \n  output: #{\"[\'#{gamma, \"\'\"delta\"\'\"}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: gamma, \' delta \';\
         \n  output: [gamma, \' delta \'];\
         \n  output: gamma, \' delta \';\
         \n  output: gamma, \' delta \';\
         \n  output: [\'gamma, \' delta \'\'];\
         \n}\n"
    );
}
