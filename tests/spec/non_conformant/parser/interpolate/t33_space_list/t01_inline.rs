//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/33_space_list/01_inline.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: \"[\"\'foo   \'\"]\"    \"bar\";\
            \n  output: #{\"[\"\'foo   \'\"]\"    \"bar\"};\
            \n  output: \"[#{\"[\"\'foo   \'\"]\"    \"bar\"}]\";\
            \n  output: \"#{\"[\"\'foo   \'\"]\"    \"bar\"}\";\
            \n  output: \'#{\"[\"\'foo   \'\"]\"    \"bar\"}\';\
            \n  output: \"[\'#{\"[\"\'foo   \'\"]\"    \"bar\"}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[\" \"foo   \" \"]\" \"bar\";\
        \n  output: [ foo    ] bar;\
        \n  output: \"[[ foo    ] bar]\";\
        \n  output: \"[ foo    ] bar\";\
        \n  output: \"[ foo    ] bar\";\
        \n  output: \"[\'[ foo    ] bar\']\";\
        \n}\
        \n"
    );
}
