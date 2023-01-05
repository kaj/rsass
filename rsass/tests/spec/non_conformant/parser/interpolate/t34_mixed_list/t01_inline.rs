//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/34_mixed_list/01_inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: \"[\"\',foo   ,   \'\"]\"    \"bar\";\
             \n  output: #{\"[\"\',foo   ,   \'\"]\"    \"bar\"};\
             \n  output: \"[#{\"[\"\',foo   ,   \'\"]\"    \"bar\"}]\";\
             \n  output: \"#{\"[\"\',foo   ,   \'\"]\"    \"bar\"}\";\
             \n  output: \'#{\"[\"\',foo   ,   \'\"]\"    \"bar\"}\';\
             \n  output: \"[\'#{\"[\"\',foo   ,   \'\"]\"    \"bar\"}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[\" \",foo   ,   \" \"]\" \"bar\";\
         \n  output: [ ,foo   ,    ] bar;\
         \n  output: \"[[ ,foo   ,    ] bar]\";\
         \n  output: \"[ ,foo   ,    ] bar\";\
         \n  output: \"[ ,foo   ,    ] bar\";\
         \n  output: \"[\'[ ,foo   ,    ] bar\']\";\
         \n}\n"
    );
}
