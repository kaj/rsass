//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/34_mixed_list/04_variable_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("04_variable_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"[\"\',foo   ,   \'\"]\"    \"bar\";\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: [ ,foo   ,    ] bar;\
         \n  output: [[ ,foo   ,    ] bar];\
         \n  output: [ ,foo   ,    ] bar;\
         \n  output: [ ,foo   ,    ] bar;\
         \n  output: [\'[ ,foo   ,    ] bar\'];\
         \n}\n"
    );
}
