//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/07_comma_list_complex/06_escape_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_escape_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: gamma, \"\'\"delta\"\'\";\
             \n.result {\
             \n  output: \"[\\#{gamma, \"\'\"delta\"\'\"}]\";\
             \n  output: \"\\#{gamma, \"\'\"delta\"\'\"}\";\
             \n  output: \'\\#{gamma, \"\'\"delta\"\'\"}\';\
             \n  output: \"[\'\\#{gamma, \"\'\"delta\"\'\"}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{gamma, \" \'\"delta\"\' \"}]\";\
         \n  output: \"#{gamma, \" \'\"delta\"\' \"}\";\
         \n  output: \'#{gamma, \"\' \"delta\" \'\"}\';\
         \n  output: \"[\'#{gamma, \" \'\"delta\"\' \"}\']\";\
         \n}\n"
    );
}
