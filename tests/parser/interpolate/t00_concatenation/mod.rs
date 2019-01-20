//! Tests auto-converted from "sass-spec/spec/parser/interpolate/00_concatenation"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

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

// Ignoring "unspaced", not expected to work yet.

/// From "sass-spec/spec/parser/interpolate/00_concatenation/unspaced-4.0"
#[test]
fn unspaced_4_0() {
    assert_eq!(
        rsass(
            "$input: literal;\n.result {\n  output: literal$input;\n  output: literal#{$input};\n  output: literal#{literal};\n  output: literal#{\"literal\"};\n  output: $input$input;\n  output: $input#{$input};\n  output: $input#{literal};\n  output: $input#{\"literal\"};\n  output: #{$input}literal;\n  output: #{$input}$input;\n  output: #{$input}#{$input};\n  output: #{$input}#{literal};\n  output: #{$input}#{\"literal\"};\n  output: #{literal}literal;\n  output: #{literal}$input;\n  output: #{literal}#{$input};\n  output: #{literal}#{literal};\n  output: #{literal}#{\"literal\"};\n  output: #{\"literal\"}literal;\n  output: #{\"literal\"}$input;\n  output: #{\"literal\"}#{$input};\n  output: #{\"literal\"}#{literal};\n  output: #{\"literal\"}#{\"literal\"};\n  output: \"literal#{$input}\";\n  output: \"literal#{literal}\";\n  output: \"literal#{\"literal\"}\";\n  output: \"#{$input}literal\";\n  output: \"#{$input}#{$input}\";\n  output: \"#{$input}#{literal}\";\n  output: \"#{$input}#{\"literal\"}\";\n  output: \"#{literal}literal\";\n  output: \"#{literal}#{$input}\";\n  output: \"#{literal}#{literal}\";\n  output: \"#{literal}#{\"literal\"}\";\n  output: \"#{\"literal\"}literal\";\n  output: \"#{\"literal\"}#{$input}\";\n  output: \"#{\"literal\"}#{literal}\";\n  output: \"#{\"literal\"}#{\"literal\"}\";\n}"
        )
        .unwrap(),
        ".result {\n  output: literal literal;\n  output: literalliteral;\n  output: literalliteral;\n  output: literalliteral;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literal literal;\n  output: literalliteral;\n  output: literal literal;\n  output: literalliteral;\n  output: literalliteral;\n  output: literalliteral;\n  output: literalliteral;\n  output: literal literal;\n  output: literalliteral;\n  output: literalliteral;\n  output: literalliteral;\n  output: literalliteral;\n  output: literal literal;\n  output: literalliteral;\n  output: literalliteral;\n  output: literalliteral;\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n  output: \"literalliteral\";\n}\n"
    );
}
