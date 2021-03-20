//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/00_concatenation/unspaced.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: literal;\
            \n.result {\
            \n  output: literal$input;\
            \n  output: literal#{$input};\
            \n  output: literal#{literal};\
            \n  output: literal#{\"literal\"};\
            \n  output: $input$input;\
            \n  output: $input#{$input};\
            \n  output: $input#{literal};\
            \n  output: $input#{\"literal\"};\
            \n  output: #{$input}literal;\
            \n  output: #{$input}$input;\
            \n  output: #{$input}#{$input};\
            \n  output: #{$input}#{literal};\
            \n  output: #{$input}#{\"literal\"};\
            \n  output: #{literal}literal;\
            \n  output: #{literal}$input;\
            \n  output: #{literal}#{$input};\
            \n  output: #{literal}#{literal};\
            \n  output: #{literal}#{\"literal\"};\
            \n  output: #{\"literal\"}literal;\
            \n  output: #{\"literal\"}$input;\
            \n  output: #{\"literal\"}#{$input};\
            \n  output: #{\"literal\"}#{literal};\
            \n  output: #{\"literal\"}#{\"literal\"};\
            \n  output: \"literal#{$input}\";\
            \n  output: \"literal#{literal}\";\
            \n  output: \"literal#{\"literal\"}\";\
            \n  output: \"#{$input}literal\";\
            \n  output: \"#{$input}#{$input}\";\
            \n  output: \"#{$input}#{literal}\";\
            \n  output: \"#{$input}#{\"literal\"}\";\
            \n  output: \"#{literal}literal\";\
            \n  output: \"#{literal}#{$input}\";\
            \n  output: \"#{literal}#{literal}\";\
            \n  output: \"#{literal}#{\"literal\"}\";\
            \n  output: \"#{\"literal\"}literal\";\
            \n  output: \"#{\"literal\"}#{$input}\";\
            \n  output: \"#{\"literal\"}#{literal}\";\
            \n  output: \"#{\"literal\"}#{\"literal\"}\";\
            \n}"
        )
        .unwrap(),
        ".result {\
        \n  output: literal literal;\
        \n  output: literalliteral;\
        \n  output: literalliteral;\
        \n  output: literalliteral;\
        \n  output: literal literal;\
        \n  output: literal literal;\
        \n  output: literal literal;\
        \n  output: literal literal;\
        \n  output: literalliteral;\
        \n  output: literal literal;\
        \n  output: literalliteral;\
        \n  output: literalliteral;\
        \n  output: literalliteral;\
        \n  output: literalliteral;\
        \n  output: literal literal;\
        \n  output: literalliteral;\
        \n  output: literalliteral;\
        \n  output: literalliteral;\
        \n  output: literalliteral;\
        \n  output: literal literal;\
        \n  output: literalliteral;\
        \n  output: literalliteral;\
        \n  output: literalliteral;\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n  output: \"literalliteral\";\
        \n}\
        \n"
    );
}
