//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/01_literal/02_variable.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("02_variable")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: literal;\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
             \n}\n"),
        ".result {\
         \n  output: literal;\
         \n  output: literal;\
         \n  output: \"[literal]\";\
         \n  output: \"literal\";\
         \n  output: \"literal\";\
         \n  output: \"[\'literal\']\";\
         \n}\n"
    );
}
