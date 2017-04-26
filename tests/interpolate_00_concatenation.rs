extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn a01() {
    check("literal  $input", "literal literal")
}

#[test]
fn a02() {
    check("literal  #{$input}", "literal literal")
}
#[test]
fn a03() {
    check("literal  #{literal}", "literal literal")
}
#[test]
fn a04() {
    check("literal  #{\"literal\"}", "literal literal")
}
#[test]
fn a05() {
    check("$input  $input", "literal literal")
}
#[test]
fn a06() {
    check("$input  #{$input}", "literal literal")
}
#[test]
fn a07() {
    check("$input  #{literal}", "literal literal")
}
#[test]
fn a08() {
    check("$input  #{\"literal\"}", "literal literal")
}
#[test]
fn a09() {
    check("#{$input}  literal", "literal literal")
}
#[test]
fn a10() {
    check("#{$input}  $input", "literal literal")
}
#[test]
fn a11() {
    check("#{$input}  #{$input}", "literal literal")
}
#[test]
fn a12() {
    check("#{$input}  #{literal}", "literal literal")
}
#[test]
fn a13() {
    check("#{$input}  #{\"literal\"}", "literal literal")
}
#[test]
fn a14() {
    check("#{literal}  literal", "literal literal")
}
#[test]
fn a15() {
    check("#{literal}  $input", "literal literal")
}
#[test]
fn a16() {
    check("#{literal}  #{$input}", "literal literal")
}
#[test]
fn a17() {
    check("#{literal}  #{literal}", "literal literal")
}
#[test]
fn a18() {
    check("#{literal}  #{\"literal\"}", "literal literal")
}
#[test]
fn a19() {
    check("#{\"literal\"}  literal", "literal literal")
}
#[test]
fn a20() {
    check("#{\"literal\"}  $input", "literal literal")
}
#[test]
fn a21() {
    check("#{\"literal\"}  #{$input}", "literal literal")
}
#[test]
fn a22() {
    check("#{\"literal\"}  #{literal}", "literal literal")
}
#[test]
fn a23() {
    check("#{\"literal\"}  #{\"literal\"}", "literal literal")
}

#[test]
fn b01() {
    check("\"literal  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn b02() {
    check("\"literal  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn b03() {
    check("\"literal  #{\"literal\"}\"", "\"literal  literal\"")
}
#[test]
fn b04_alt() {
    check("\"#{$input}\"", "\"literal\"")
}
#[test]
fn b04() {
    check("\"#{$input}  literal\"", "\"literal  literal\"")
}
#[test]
fn b05() {
    check("\"#{$input}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn b06() {
    check("\"#{$input}  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn b07() {
    check("\"#{$input}  #{\"literal\"}\"", "\"literal  literal\"")
}
#[test]
fn b08() {
    check("\"#{literal}  literal\"", "\"literal  literal\"")
}
#[test]
fn b09() {
    check("\"#{literal}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn b10() {
    check("\"#{literal}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn b11() {
    check("\"#{literal}  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn b12() {
    check("\"#{literal}  #{\"literal\"}\"", "\"literal  literal\"")
}
#[test]
fn b13() {
    check("\"#{\"literal\"}  literal\"", "\"literal  literal\"")
}
#[test]
fn b14() {
    check("\"#{\"literal\"}  #{$input}\"", "\"literal  literal\"")
}
#[test]
fn b15() {
    check("\"#{\"literal\"}  #{literal}\"", "\"literal  literal\"")
}
#[test]
fn b16() {
    check("\"#{\"literal\"}  #{\"literal\"}\"", "\"literal  literal\"")
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
