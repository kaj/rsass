//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/04_variable_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \\0_\\a_\\A;\
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
        \n  output: \\0 _\\a _\\a ;\
        \n  output: [\\0 _\\a _\\a ];\
        \n  output: \\0 _\\a _\\a ;\
        \n  output: \\0 _\\a _\\a ;\
        \n  output: [\'\\0 _\\a _\\a \'];\
        \n}\
        \n"
    );
}
