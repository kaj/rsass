//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/07_comma_list_complex/06_escape_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: gamma, \"\'\"delta\"\'\";\
            \n.result {\
            \n  output: \"[\\#{gamma, \"\'\"delta\"\'\"}]\";\
            \n  output: \"\\#{gamma, \"\'\"delta\"\'\"}\";\
            \n  output: \'\\#{gamma, \"\'\"delta\"\'\"}\';\
            \n  output: \"[\'\\#{gamma, \"\'\"delta\"\'\"}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{gamma, \" \'\"delta\"\' \"}]\";\
        \n  output: \"#{gamma, \" \'\"delta\"\' \"}\";\
        \n  output: \'#{gamma, \"\' \"delta\" \'\"}\';\
        \n  output: \"[\'#{gamma, \" \'\"delta\"\' \"}\']\";\
        \n}\
        \n"
    );
}
