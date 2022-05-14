//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/13_escaped_single_quoted/02_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("02_variable")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \'l\\\\ite\\ral\';\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"l\\\\iteral\";\
         \n  output: l\\iteral;\
         \n  output: \"[l\\\\iteral]\";\
         \n  output: \"l\\\\iteral\";\
         \n  output: \"l\\\\iteral\";\
         \n  output: \"[\'l\\\\iteral\']\";\
         \n}\n"
    );
}
