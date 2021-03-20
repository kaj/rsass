//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/06_space_list_complex/01_inline.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: gamme \"\'\"delta\"\'\";\
            \n  output: #{gamme \"\'\"delta\"\'\"};\
            \n  output: \"[#{gamme \"\'\"delta\"\'\"}]\";\
            \n  output: \"#{gamme \"\'\"delta\"\'\"}\";\
            \n  output: \'#{gamme \"\'\"delta\"\'\"}\';\
            \n  output: \"[\'#{gamme \"\'\"delta\"\'\"}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: gamme \"\'\" delta \"\'\";\
        \n  output: gamme \' delta \';\
        \n  output: \"[gamme \' delta \']\";\
        \n  output: \"gamme \' delta \'\";\
        \n  output: \"gamme \' delta \'\";\
        \n  output: \"[\'gamme \' delta \'\']\";\
        \n}\
        \n"
    );
}
