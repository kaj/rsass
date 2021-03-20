//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/03_inline_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: #{#{\\0_\\a_\\A}};\
            \n  output: #{\"[#{\\0_\\a_\\A}]\"};\
            \n  output: #{\"#{\\0_\\a_\\A}\"};\
            \n  output: #{\'#{\\0_\\a_\\A}\'};\
            \n  output: #{\"[\'#{\\0_\\a_\\A}\']\"};\
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
