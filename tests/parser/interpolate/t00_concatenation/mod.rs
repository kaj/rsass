//! Tests auto-converted from "sass-spec/spec/parser/interpolate/00_concatenation"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::precision;

/// From "sass-spec/spec/parser/interpolate/00_concatenation/spaced"
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

// Ignoring "unspaced", end_version is 3.5.

// Ignoring "unspaced-4.0", start_version is 4.0.
