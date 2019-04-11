//! Tests auto-converted from "sass-spec/spec/parser/interpolate/00_concatenation"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/parser/interpolate/00_concatenation/spaced.hrx"
#[test]
fn spaced() {
    assert_eq!(
        rsass(
            "$input: literal;\n.result {\n  output: literal  $input;\n  output: literal  #{$input};\n  output: literal  #{literal};\n  output: literal  #{\"literal\"};\n  output: $input  $input;\n  output: $input  #{$input};\n  output: $input  #{literal};\n  output: $input  #{\"literal\"};\n  output: #{$input}  literal;\n  output: #{$input}  $input;\n  output: #{$input}  #{$input};\n  output: #{$input}  #{literal};\n  output: #{$input}  #{\"literal\"};\n  output: #{literal}  literal;\n  output: #{literal}  $input;\n  output: #{literal}  #{$input};\n  output: #{literal}  #{literal};\n  output: #{literal}  #{\"literal\"};\n  output: #{\"literal\"}  literal;\n  output: #{\"literal\"}  $input;\n  output: #{\"literal\"}  #{$input};\n  output: #{\"literal\"}  #{literal};\n  output: #{\"literal\"}  #{\"literal\"};\n  output: \"literal  #{$input}\";\n  output: \"literal  #{literal}\";\n  output: \"literal  #{\"literal\"}\";\n  output: \"#{$input}  literal\";\n  output: \"#{$input}  #{$input}\";\n  output: \"#{$input}  #{literal}\";\n  output: \"#{$input}  #{\"literal\"}\";\n  output: \"#{literal}  literal\";\n  output: \"#{literal}  #{$input}\";\n  output: \"#{literal}  #{literal}\";\n  output: \"#{literal}  #{\"literal\"}\";\n  output: \"#{\"literal\"}  literal\";\n  output: \"#{\"literal\"}  #{$input}\";\n  output: \"#{\"literal\"}  #{literal}\";\n  output: \"#{\"literal\"}  #{\"literal\"}\";\n}"
        )
        .unwrap(),
        ".result {\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n  output: \"literal  literal\";\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/00_concatenation/unspaced-4.0.hrx"

// Ignoring "unspaced_4_0", start_version is 4.0.

// From "sass-spec/spec/parser/interpolate/00_concatenation/unspaced.hrx"

// Ignoring "unspaced", end_version is 3.5.
