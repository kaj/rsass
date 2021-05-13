//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/03_single_quoted/01_inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: \'squoted\';\
             \n  output: #{\'squoted\'};\
             \n  output: \"[#{\'squoted\'}]\";\
             \n  output: \"#{\'squoted\'}\";\
             \n  output: \'#{\'squoted\'}\';\
             \n  output: \"[\'#{\'squoted\'}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"squoted\";\
         \n  output: squoted;\
         \n  output: \"[squoted]\";\
         \n  output: \"squoted\";\
         \n  output: \"squoted\";\
         \n  output: \"[\'squoted\']\";\
         \n}\n"
    );
}
