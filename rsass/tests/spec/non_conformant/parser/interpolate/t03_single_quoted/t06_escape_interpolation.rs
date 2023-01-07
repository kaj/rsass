//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/03_single_quoted/06_escape_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_escape_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \'squoted\';\
             \n.result {\
             \n  output: \"[\\#{\'squoted\'}]\";\
             \n  output: \"\\#{\'squoted\'}\";\
             \n  output: \'\\#{\'squoted\'}\';\
             \n  output: \"[\'\\#{\'squoted\'}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{\'squoted\'}]\";\
         \n  output: \"#{\'squoted\'}\";\
         \n  output: \"#{\" squoted \"}\";\
         \n  output: \"[\'#{\'squoted\'}\']\";\
         \n}\n"
    );
}
