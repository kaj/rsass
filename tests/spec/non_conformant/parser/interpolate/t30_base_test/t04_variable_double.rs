//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/30_base_test/04_variable_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"foo#{\'ba\' + \'r\'}baz\";\
            \n.result {\
            \n  output: #{#{$input}};\
            \n  output: #{\"[#{$input}]\"};\
            \n  output: #{\"#{$input}\"};\
            \n  output: #{\'#{$input}\'};\
            \n  output: #{\"[\'#{$input}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: foobarbaz;\
        \n  output: [foobarbaz];\
        \n  output: foobarbaz;\
        \n  output: foobarbaz;\
        \n  output: [\'foobarbaz\'];\
        \n}\
        \n"
    );
}
