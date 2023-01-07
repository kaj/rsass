//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/06_space_list_complex/06_escape_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_escape_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: gamme \"\'\"delta\"\'\";\
             \n.result {\
             \n  output: \"[\\#{gamme \"\'\"delta\"\'\"}]\";\
             \n  output: \"\\#{gamme \"\'\"delta\"\'\"}\";\
             \n  output: \'\\#{gamme \"\'\"delta\"\'\"}\';\
             \n  output: \"[\'\\#{gamme \"\'\"delta\"\'\"}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{gamme \" \'\"delta\"\' \"}]\";\
         \n  output: \"#{gamme \" \'\"delta\"\' \"}\";\
         \n  output: \'#{gamme \"\' \"delta\" \'\"}\';\
         \n  output: \"[\'#{gamme \" \'\"delta\"\' \"}\']\";\
         \n}\n"
    );
}
