//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/32_comma_list/03_inline_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("03_inline_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: #{#{\"[\"\',foo,   \'\"]\"}};\
             \n  output: #{\"[#{\"[\"\',foo,   \'\"]\"}]\"};\
             \n  output: #{\"#{\"[\"\',foo,   \'\"]\"}\"};\
             \n  output: #{\'#{\"[\"\',foo,   \'\"]\"}\'};\
             \n  output: #{\"[\'#{\"[\"\',foo,   \'\"]\"}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: [ ,foo,    ];\
         \n  output: [[ ,foo,    ]];\
         \n  output: [ ,foo,    ];\
         \n  output: [ ,foo,    ];\
         \n  output: [\'[ ,foo,    ]\'];\
         \n}\n"
    );
}
