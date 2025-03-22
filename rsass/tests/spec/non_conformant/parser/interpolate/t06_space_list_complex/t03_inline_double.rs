//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/06_space_list_complex/03_inline_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("03_inline_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: #{#{gamme \"\'\"delta\"\'\"}};\
             \n  output: #{\"[#{gamme \"\'\"delta\"\'\"}]\"};\
             \n  output: #{\"#{gamme \"\'\"delta\"\'\"}\"};\
             \n  output: #{\'#{gamme \"\'\"delta\"\'\"}\'};\
             \n  output: #{\"[\'#{gamme \"\'\"delta\"\'\"}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: gamme \' delta \';\
         \n  output: [gamme \' delta \'];\
         \n  output: gamme \' delta \';\
         \n  output: gamme \' delta \';\
         \n  output: [\'gamme \' delta \'\'];\
         \n}\n"
    );
}
