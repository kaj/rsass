//! Tests auto-converted from "sass-spec/spec/parser/interpolate/00_concatenation"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/parser/interpolate/00_concatenation/spaced.hrx"
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

// Ignoring "unspaced-4.0.hrx", start_version is 4.0.

// Ignoring "unspaced.hrx", end_version is 3.5.
