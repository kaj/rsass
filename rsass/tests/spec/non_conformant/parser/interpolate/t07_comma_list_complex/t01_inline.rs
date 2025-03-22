//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/07_comma_list_complex/01_inline.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: gamma, \"\'\"delta\"\'\";\
             \n  output: #{gamma, \"\'\"delta\"\'\"};\
             \n  output: \"[#{gamma, \"\'\"delta\"\'\"}]\";\
             \n  output: \"#{gamma, \"\'\"delta\"\'\"}\";\
             \n  output: \'#{gamma, \"\'\"delta\"\'\"}\';\
             \n  output: \"[\'#{gamma, \"\'\"delta\"\'\"}\']\";\
             \n}\n"),
        ".result {\
         \n  output: gamma, \"\'\" delta \"\'\";\
         \n  output: gamma, \' delta \';\
         \n  output: \"[gamma, \' delta \']\";\
         \n  output: \"gamma, \' delta \'\";\
         \n  output: \"gamma, \' delta \'\";\
         \n  output: \"[\'gamma, \' delta \'\']\";\
         \n}\n"
    );
}
