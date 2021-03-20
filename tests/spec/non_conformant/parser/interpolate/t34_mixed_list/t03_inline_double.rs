//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/34_mixed_list/03_inline_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: #{#{\"[\"\',foo   ,   \'\"]\"    \"bar\"}};\
            \n  output: #{\"[#{\"[\"\',foo   ,   \'\"]\"    \"bar\"}]\"};\
            \n  output: #{\"#{\"[\"\',foo   ,   \'\"]\"    \"bar\"}\"};\
            \n  output: #{\'#{\"[\"\',foo   ,   \'\"]\"    \"bar\"}\'};\
            \n  output: #{\"[\'#{\"[\"\',foo   ,   \'\"]\"    \"bar\"}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: [ ,foo   ,    ] bar;\
        \n  output: [[ ,foo   ,    ] bar];\
        \n  output: [ ,foo   ,    ] bar;\
        \n  output: [ ,foo   ,    ] bar;\
        \n  output: [\'[ ,foo   ,    ] bar\'];\
        \n}\
        \n"
    );
}
