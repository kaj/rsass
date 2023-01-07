//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/00_concatenation/spaced.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("spaced")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: literal;\
             \n.result {\
             \n  output: literal  $input;\
             \n  output: literal  #{$input};\
             \n  output: literal  #{literal};\
             \n  output: literal  #{\"literal\"};\
             \n  output: $input  $input;\
             \n  output: $input  #{$input};\
             \n  output: $input  #{literal};\
             \n  output: $input  #{\"literal\"};\
             \n  output: #{$input}  literal;\
             \n  output: #{$input}  $input;\
             \n  output: #{$input}  #{$input};\
             \n  output: #{$input}  #{literal};\
             \n  output: #{$input}  #{\"literal\"};\
             \n  output: #{literal}  literal;\
             \n  output: #{literal}  $input;\
             \n  output: #{literal}  #{$input};\
             \n  output: #{literal}  #{literal};\
             \n  output: #{literal}  #{\"literal\"};\
             \n  output: #{\"literal\"}  literal;\
             \n  output: #{\"literal\"}  $input;\
             \n  output: #{\"literal\"}  #{$input};\
             \n  output: #{\"literal\"}  #{literal};\
             \n  output: #{\"literal\"}  #{\"literal\"};\
             \n  output: \"literal  #{$input}\";\
             \n  output: \"literal  #{literal}\";\
             \n  output: \"literal  #{\"literal\"}\";\
             \n  output: \"#{$input}  literal\";\
             \n  output: \"#{$input}  #{$input}\";\
             \n  output: \"#{$input}  #{literal}\";\
             \n  output: \"#{$input}  #{\"literal\"}\";\
             \n  output: \"#{literal}  literal\";\
             \n  output: \"#{literal}  #{$input}\";\
             \n  output: \"#{literal}  #{literal}\";\
             \n  output: \"#{literal}  #{\"literal\"}\";\
             \n  output: \"#{\"literal\"}  literal\";\
             \n  output: \"#{\"literal\"}  #{$input}\";\
             \n  output: \"#{\"literal\"}  #{literal}\";\
             \n  output: \"#{\"literal\"}  #{\"literal\"}\";\
             \n}"),
        ".result {\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: literal literal;\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n  output: \"literal  literal\";\
         \n}\n"
    );
}
