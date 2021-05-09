//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/32_comma_list/04_variable_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"[\"\',foo,   \'\"]\";\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
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
