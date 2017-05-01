/// Tests from `spec/parser/interpolate/00_concatenation` split up
/// into the individual properties to be tested.
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn spaced_a01() {
    check("literal  $input", "literal literal")
}

#[test]
fn spaced_a02() {
    check("literal  #{$input}", "literal literal")
}
#[test]
fn spaced_a03() {
    check("literal  #{literal}", "literal literal")
}
#[test]
fn spaced_a04() {
    check("literal  #{\"literal\"}", "literal literal")
}
#[test]
fn spaced_a05() {
    check("$input  $input", "literal literal")
}
#[test]
fn spaced_a06() {
    check("$input  #{$input}", "literal literal")
}
#[test]
fn spaced_a07() {
    check("$input  #{literal}", "literal literal")
}
#[test]
fn spaced_a08() {
    check("$input  #{\"literal\"}", "literal literal")
}
#[test]
fn spaced_a09() {
    check("#{$input}  literal", "literal literal")
}
#[test]
fn spaced_a10() {
    check("#{$input}  $input", "literal literal")
}
#[test]
fn spaced_a11() {
    check("#{$input}  #{$input}", "literal literal")
}
#[test]
fn spaced_a12() {
    check("#{$input}  #{literal}", "literal literal")
}
#[test]
fn spaced_a13() {
    check("#{$input}  #{\"literal\"}", "literal literal")
}
#[test]
fn spaced_a14() {
    check("#{literal}  literal", "literal literal")
}
#[test]
fn spaced_a15() {
    check("#{literal}  $input", "literal literal")
}
#[test]
fn spaced_a16() {
    check("#{literal}  #{$input}", "literal literal")
}
#[test]
fn spaced_a17() {
    check("#{literal}  #{literal}", "literal literal")
}
#[test]
fn spaced_a18() {
    check("#{literal}  #{\"literal\"}", "literal literal")
}
#[test]
fn spaced_a19() {
    check("#{\"literal\"}  literal", "literal literal")
}
#[test]
fn spaced_a20() {
    check("#{\"literal\"}  $input", "literal literal")
}
#[test]
fn spaced_a21() {
    check("#{\"literal\"}  #{$input}", "literal literal")
}
#[test]
fn spaced_a22() {
    check("#{\"literal\"}  #{literal}", "literal literal")
}
#[test]
fn spaced_a23() {
    check("#{\"literal\"}  #{\"literal\"}", "literal literal")
}

#[test]
fn spaced_b01() {
    check("\"literal  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b02() {
    check("\"literal  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b03() {
    check("\"literal  #{\"literal\"}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b04_alt() {
    check("\"#{$input}\"", "\"literal\"")
}
#[test]
fn spaced_b04() {
    check("\"#{$input}  literal\"", "\"literal  literal\"")
}
#[test]
fn spaced_b05() {
    check("\"#{$input}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b06() {
    check("\"#{$input}  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b07() {
    check("\"#{$input}  #{\"literal\"}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b08() {
    check("\"#{literal}  literal\"", "\"literal  literal\"")
}
#[test]
fn spaced_b09() {
    check("\"#{literal}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b10() {
    check("\"#{literal}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b11() {
    check("\"#{literal}  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b12() {
    check("\"#{literal}  #{\"literal\"}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b13() {
    check("\"#{\"literal\"}  literal\"", "\"literal  literal\"")
}
#[test]
fn spaced_b14() {
    check("\"#{\"literal\"}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b15() {
    check("\"#{\"literal\"}  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b16() {
    check("\"#{\"literal\"}  #{\"literal\"}\"", "\"literal  literal\"")
}

#[test]
fn unspaced_4_0_a1() {
    check("literal$input", "literal literal")
}
#[test]
fn unspaced_4_0_a2() {
    check("literal#{$input}", "literalliteral")
}
#[test]
fn unspaced_4_0_a3() {
    check("literal#{literal}", "literalliteral")
}
#[test]
fn unspaced_4_0_a4() {
    check("literal#{\"literal\"}", "literalliteral")
}

#[test]
fn unspaced_4_0_b1() {
    check("$input$input", "literal literal")
}
#[test]
fn unspaced_4_0_b2() {
    check("$input#{$input}", "literal literal")
}
#[test]
fn unspaced_4_0_b3() {
    check("$input#{literal}", "literal literal")
}
#[test]
fn unspaced_4_0_b4() {
    check("$input#{\"literal\"}", "literal literal")
}

#[test]
fn unspaced_4_0_c1() {
    check("#{$input}literal", "literalliteral")
}
#[test]
fn unspaced_4_0_c2() {
    check("#{$input}$input", "literal literal")
}
#[test]
fn unspaced_4_0_c3() {
    check("#{$input}#{$input}", "literalliteral")
}
#[test]
fn unspaced_4_0_c4() {
    check("#{$input}#{literal}", "literalliteral")
}
#[test]
fn unspaced_4_0_c5() {
    check("#{$input}#{\"literal\"}", "literalliteral")
}

#[test]
fn unspaced_4_0_d1() {
    check("#{literal}literal", "literalliteral")
}
#[test]
fn unspaced_4_0_d2() {
    check("#{literal}$input", "literal literal")
}
#[test]
fn unspaced_4_0_d3() {
    check("#{literal}#{$input}", "literalliteral")
}
#[test]
fn unspaced_4_0_d4() {
    check("#{literal}#{literal}", "literalliteral")
}
#[test]
fn unspaced_4_0_d5() {
    check("#{literal}#{\"literal\"}", "literalliteral")
}

#[test]
fn unspaced_4_0_e1() {
    check("#{\"literal\"}literal", "literalliteral")
}
#[test]
fn unspaced_4_0_e2() {
    check("#{\"literal\"}$input", "literal literal")
}
#[test]
fn unspaced_4_0_e3() {
    check("#{\"literal\"}#{$input}", "literalliteral")
}
#[test]
fn unspaced_4_0_e4() {
    check("#{\"literal\"}#{literal}", "literalliteral")
}
#[test]
fn unspaced_4_0_e5() {
    check("#{\"literal\"}#{\"literal\"}", "literalliteral")
}

#[test]
fn unspaced_4_0_f1() {
    check("literal#{$input}", "literalliteral")
}
#[test]
fn unspaced_4_0_f2() {
    check("literal#{literal}", "literalliteral")
}
#[test]
fn unspaced_4_0_f3() {
    check("literal#{\"literal\"}", "literalliteral")
}

#[test]
fn unspaced_4_0_g1() {
    check("\"#{$input}literal\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_g2() {
    check("\"#{$input}#{$input}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_g3() {
    check("\"#{$input}#{literal}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_g4() {
    check("\"#{$input}#{\"literal\"}\"", "\"literalliteral\"")
}

#[test]
fn unspaced_4_0_h1() {
    check("\"#{literal}literal\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_h2() {
    check("\"#{literal}#{$input}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_h3() {
    check("\"#{literal}#{literal}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_h4() {
    check("\"#{literal}#{\"literal\"}\"", "\"literalliteral\"")
}

#[test]
fn unspaced_4_0_i1() {
    check("\"#{\"literal\"}literal\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_i2() {
    check("\"#{\"literal\"}#{$input}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_i3() {
    check("\"#{\"literal\"}#{literal}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_i4() {
    check("\"#{\"literal\"}#{\"literal\"}\"", "\"literalliteral\"")
}

fn check(value: &str, expected: &str) {
    assert_eq!(compile_scss(format!("$input: literal;\n\
                                     a {{ x: {}; }}",
                                    value)
                                    .as_bytes(),
                            OutputStyle::Normal)
                       .and_then(|s| Ok(String::from_utf8(s)?))
                       .unwrap(),
               format!("a {{\n  x: {};\n}}\n", expected));
}
