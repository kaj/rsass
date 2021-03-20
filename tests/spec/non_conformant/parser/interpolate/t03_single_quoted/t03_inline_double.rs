//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/03_single_quoted/03_inline_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: #{#{\'squoted\'}};\
            \n  output: #{\"[#{\'squoted\'}]\"};\
            \n  output: #{\"#{\'squoted\'}\"};\
            \n  output: #{\'#{\'squoted\'}\'};\
            \n  output: #{\"[\'#{\'squoted\'}\']\"};\
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
