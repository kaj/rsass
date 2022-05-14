//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/06_space_list_complex/01_inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: gamme \"\'\"delta\"\'\";\
             \n  output: #{gamme \"\'\"delta\"\'\"};\
             \n  output: \"[#{gamme \"\'\"delta\"\'\"}]\";\
             \n  output: \"#{gamme \"\'\"delta\"\'\"}\";\
             \n  output: \'#{gamme \"\'\"delta\"\'\"}\';\
             \n  output: \"[\'#{gamme \"\'\"delta\"\'\"}\']\";\
             \n}\n"),
        ".result {\
         \n  output: gamme \"\'\" delta \"\'\";\
         \n  output: gamme \' delta \';\
         \n  output: \"[gamme \' delta \']\";\
         \n  output: \"gamme \' delta \'\";\
         \n  output: \"gamme \' delta \'\";\
         \n  output: \"[\'gamme \' delta \'\']\";\
         \n}\n"
    );
}
