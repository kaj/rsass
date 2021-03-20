//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/03_single_quoted/04_variable_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \'squoted\';\
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
        \n  output: squoted;\
        \n  output: [squoted];\
        \n  output: squoted;\
        \n  output: squoted;\
        \n  output: [\'squoted\'];\
        \n}\
        \n"
    );
}
