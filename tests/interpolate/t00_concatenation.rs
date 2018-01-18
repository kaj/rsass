//! Tests from `spec/parser/interpolate/00_concatenation` split up
//! into the individual properties to be tested.

use rsass::{parse_value_data, GlobalScope, Scope};

#[test]
fn spaced_a01() {
    check_value("literal  $input", "literal literal")
}

#[test]
fn spaced_a02() {
    check_value("literal  #{$input}", "literal literal")
}
#[test]
fn spaced_a03() {
    check_value("literal  #{literal}", "literal literal")
}
#[test]
fn spaced_a04() {
    check_value("literal  #{\"literal\"}", "literal literal")
}
#[test]
fn spaced_a05() {
    check_value("$input  $input", "literal literal")
}
#[test]
fn spaced_a06() {
    check_value("$input  #{$input}", "literal literal")
}
#[test]
fn spaced_a07() {
    check_value("$input  #{literal}", "literal literal")
}
#[test]
fn spaced_a08() {
    check_value("$input  #{\"literal\"}", "literal literal")
}
#[test]
fn spaced_a09() {
    check_value("#{$input}  literal", "literal literal")
}
#[test]
fn spaced_a10() {
    check_value("#{$input}  $input", "literal literal")
}
#[test]
fn spaced_a11() {
    check_value("#{$input}  #{$input}", "literal literal")
}
#[test]
fn spaced_a12() {
    check_value("#{$input}  #{literal}", "literal literal")
}
#[test]
fn spaced_a13() {
    check_value("#{$input}  #{\"literal\"}", "literal literal")
}
#[test]
fn spaced_a14() {
    check_value("#{literal}  literal", "literal literal")
}
#[test]
fn spaced_a15() {
    check_value("#{literal}  $input", "literal literal")
}
#[test]
fn spaced_a16() {
    check_value("#{literal}  #{$input}", "literal literal")
}
#[test]
fn spaced_a17() {
    check_value("#{literal}  #{literal}", "literal literal")
}
#[test]
fn spaced_a18() {
    check_value("#{literal}  #{\"literal\"}", "literal literal")
}
#[test]
fn spaced_a19() {
    check_value("#{\"literal\"}  literal", "literal literal")
}
#[test]
fn spaced_a20() {
    check_value("#{\"literal\"}  $input", "literal literal")
}
#[test]
fn spaced_a21() {
    check_value("#{\"literal\"}  #{$input}", "literal literal")
}
#[test]
fn spaced_a22() {
    check_value("#{\"literal\"}  #{literal}", "literal literal")
}
#[test]
fn spaced_a23() {
    check_value("#{\"literal\"}  #{\"literal\"}", "literal literal")
}

#[test]
fn spaced_b01() {
    check_value("\"literal  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b02() {
    check_value("\"literal  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b03() {
    check_value("\"literal  #{\"literal\"}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b04_alt() {
    check_value("\"#{$input}\"", "\"literal\"")
}
#[test]
fn spaced_b04() {
    check_value("\"#{$input}  literal\"", "\"literal  literal\"")
}
#[test]
fn spaced_b05() {
    check_value("\"#{$input}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b06() {
    check_value("\"#{$input}  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b07() {
    check_value("\"#{$input}  #{\"literal\"}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b08() {
    check_value("\"#{literal}  literal\"", "\"literal  literal\"")
}
#[test]
fn spaced_b09() {
    check_value("\"#{literal}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b10() {
    check_value("\"#{literal}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b11() {
    check_value("\"#{literal}  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b12() {
    check_value("\"#{literal}  #{\"literal\"}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b13() {
    check_value("\"#{\"literal\"}  literal\"", "\"literal  literal\"")
}
#[test]
fn spaced_b14() {
    check_value("\"#{\"literal\"}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b15() {
    check_value("\"#{\"literal\"}  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn spaced_b16() {
    check_value("\"#{\"literal\"}  #{\"literal\"}\"", "\"literal  literal\"")
}

#[test]
fn unspaced_4_0_a1() {
    check_value("literal$input", "literal literal")
}
#[test]
fn unspaced_4_0_a2() {
    check_value("literal#{$input}", "literalliteral")
}
#[test]
fn unspaced_4_0_a3() {
    check_value("literal#{literal}", "literalliteral")
}
#[test]
fn unspaced_4_0_a4() {
    check_value("literal#{\"literal\"}", "literalliteral")
}

#[test]
fn unspaced_4_0_b1() {
    check_value("$input$input", "literal literal")
}
#[test]
fn unspaced_4_0_b2() {
    check_value("$input#{$input}", "literal literal")
}
#[test]
fn unspaced_4_0_b3() {
    check_value("$input#{literal}", "literal literal")
}
#[test]
fn unspaced_4_0_b4() {
    check_value("$input#{\"literal\"}", "literal literal")
}

#[test]
fn unspaced_4_0_c1() {
    check_value("#{$input}literal", "literalliteral")
}
#[test]
fn unspaced_4_0_c2() {
    check_value("#{$input}$input", "literal literal")
}
#[test]
fn unspaced_4_0_c3() {
    check_value("#{$input}#{$input}", "literalliteral")
}
#[test]
fn unspaced_4_0_c4() {
    check_value("#{$input}#{literal}", "literalliteral")
}
#[test]
fn unspaced_4_0_c5() {
    check_value("#{$input}#{\"literal\"}", "literalliteral")
}

#[test]
fn unspaced_4_0_d1() {
    check_value("#{literal}literal", "literalliteral")
}
#[test]
fn unspaced_4_0_d2() {
    check_value("#{literal}$input", "literal literal")
}
#[test]
fn unspaced_4_0_d3() {
    check_value("#{literal}#{$input}", "literalliteral")
}
#[test]
fn unspaced_4_0_d4() {
    check_value("#{literal}#{literal}", "literalliteral")
}
#[test]
fn unspaced_4_0_d5() {
    check_value("#{literal}#{\"literal\"}", "literalliteral")
}

#[test]
fn unspaced_4_0_e1() {
    check_value("#{\"literal\"}literal", "literalliteral")
}
#[test]
fn unspaced_4_0_e2() {
    check_value("#{\"literal\"}$input", "literal literal")
}
#[test]
fn unspaced_4_0_e3() {
    check_value("#{\"literal\"}#{$input}", "literalliteral")
}
#[test]
fn unspaced_4_0_e4() {
    check_value("#{\"literal\"}#{literal}", "literalliteral")
}
#[test]
fn unspaced_4_0_e5() {
    check_value("#{\"literal\"}#{\"literal\"}", "literalliteral")
}

#[test]
fn unspaced_4_0_f1() {
    check_value("literal#{$input}", "literalliteral")
}
#[test]
fn unspaced_4_0_f2() {
    check_value("literal#{literal}", "literalliteral")
}
#[test]
fn unspaced_4_0_f3() {
    check_value("literal#{\"literal\"}", "literalliteral")
}

#[test]
fn unspaced_4_0_g1() {
    check_value("\"#{$input}literal\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_g2() {
    check_value("\"#{$input}#{$input}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_g3() {
    check_value("\"#{$input}#{literal}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_g4() {
    check_value("\"#{$input}#{\"literal\"}\"", "\"literalliteral\"")
}

#[test]
fn unspaced_4_0_h1() {
    check_value("\"#{literal}literal\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_h2() {
    check_value("\"#{literal}#{$input}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_h3() {
    check_value("\"#{literal}#{literal}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_h4() {
    check_value("\"#{literal}#{\"literal\"}\"", "\"literalliteral\"")
}

#[test]
fn unspaced_4_0_i1() {
    check_value("\"#{\"literal\"}literal\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_i2() {
    check_value("\"#{\"literal\"}#{$input}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_i3() {
    check_value("\"#{\"literal\"}#{literal}\"", "\"literalliteral\"")
}
#[test]
fn unspaced_4_0_i4() {
    check_value("\"#{\"literal\"}#{\"literal\"}\"", "\"literalliteral\"")
}

fn check_value(value: &str, expected: &str) {
    let mut scope = GlobalScope::new();
    let input_value = parse_value_data(b"literal").unwrap().evaluate(&scope);
    scope.define("input", &input_value);
    let value = parse_value_data(value.as_bytes()).unwrap();
    assert_eq!(format!("{}", value.evaluate(&scope)), expected)
}
