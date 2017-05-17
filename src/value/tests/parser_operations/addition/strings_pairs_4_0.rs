//! Tests from
//! `sass-spec/spec/parser/operations/addition/strings/pairs-4.0`.
//!
//! All tests marked `ignore` here are TODO.

use super::super::super::super::value_expression;
use std::str::from_utf8;
use variablescope::GlobalScope;

#[test]
fn t01() {
    check("literal+literal", "literalliteral")
}
#[test]
#[ignore]
fn t02() {
    check("literal +literal", "literalliteral")
}
#[test]
fn t03() {
    check("literal+ literal", "literalliteral")
}
#[test]
fn t04() {
    check("literal + literal", "literalliteral")
}
#[test]
fn t05() {
    check("literal+\"quoted\"", "literalquoted")
}
#[test]
#[ignore]
fn t06() {
    check("literal +\"quoted\"", "literalquoted")
}
#[test]
fn t07() {
    check("literal+ \"quoted\"", "literalquoted")
}
#[test]
fn t08() {
    check("literal + \"quoted\"", "literalquoted")
}
#[test]
fn t09() {
    check("literal+#{interpolant}", "literalinterpolant")
}
#[test]
#[ignore]
fn t10() {
    check("literal +#{interpolant}", "literalinterpolant")
}
#[test]
fn t11() {
    check("literal+ #{interpolant}", "literalinterpolant")
}
#[test]
fn t12() {
    check("literal + #{interpolant}", "literalinterpolant")
}
#[test]
fn t13() {
    check("literal+lschema_#{ritlp}", "literallschema_ritlp")
}
#[test]
#[ignore]
fn t14() {
    check("literal +lschema_#{ritlp}", "literallschema_ritlp")
}
#[test]
fn t15() {
    check("literal+ lschema_#{ritlp}", "literallschema_ritlp")
}
#[test]
fn t16() {
    check("literal + lschema_#{ritlp}", "literallschema_ritlp")
}
#[test]
fn t17() {
    check("literal+#{litlp}_rschema", "literallitlp_rschema")
}
#[test]
#[ignore]
fn t18() {
    check("literal +#{litlp}_rschema", "literallitlp_rschema")
}
#[test]
fn t19() {
    check("literal+ #{litlp}_rschema", "literallitlp_rschema")
}
#[test]
fn t20() {
    check("literal + #{litlp}_rschema", "literallitlp_rschema")
}
#[test]
fn t21() {
    check("\"quoted\"+\"quoted\"", "\"quotedquoted\"")
}
#[test]
#[ignore]
fn t22() {
    check("\"quoted\" +\"quoted\"", "\"quotedquoted\"")
}
#[test]
fn t23() {
    check("\"quoted\"+ \"quoted\"", "\"quotedquoted\"")
}
#[test]
fn t24() {
    check("\"quoted\" + \"quoted\"", "\"quotedquoted\"")
}
#[test]
fn t25() {
    check("\"quoted\"+#{interpolant}", "\"quotedinterpolant\"")
}
#[test]
#[ignore]
fn t26() {
    check("\"quoted\" +#{interpolant}", "\"quotedinterpolant\"")
}
#[test]
fn t27() {
    check("\"quoted\"+ #{interpolant}", "\"quotedinterpolant\"")
}
#[test]
fn t28() {
    check("\"quoted\" + #{interpolant}", "\"quotedinterpolant\"")
}
#[test]
fn t29() {
    check("\"quoted\"+lschema_#{ritlp}", "\"quotedlschema_ritlp\"")
}
#[test]
#[ignore]
fn t30() {
    check("\"quoted\" +lschema_#{ritlp}", "\"quotedlschema_ritlp\"")
}
#[test]
fn t31() {
    check("\"quoted\"+ lschema_#{ritlp}", "\"quotedlschema_ritlp\"")
}
#[test]
fn t32() {
    check("\"quoted\" + lschema_#{ritlp}", "\"quotedlschema_ritlp\"")
}
#[test]
fn t33() {
    check("\"quoted\"+#{litlp}_rschema", "\"quotedlitlp_rschema\"")
}
#[test]
#[ignore]
fn t34() {
    check("\"quoted\" +#{litlp}_rschema", "\"quotedlitlp_rschema\"")
}
#[test]
fn t35() {
    check("\"quoted\"+ #{litlp}_rschema", "\"quotedlitlp_rschema\"")
}
#[test]
fn t36() {
    check("\"quoted\" + #{litlp}_rschema", "\"quotedlitlp_rschema\"")
}
#[test]
fn t37() {
    check("#{interpolant}+#{interpolant}", "interpolantinterpolant")
}
#[test]
#[ignore]
fn t38() {
    check("#{interpolant} +#{interpolant}", "interpolantinterpolant")
}
#[test]
fn t39() {
    check("#{interpolant}+ #{interpolant}", "interpolantinterpolant")
}
#[test]
fn t40() {
    check("#{interpolant} + #{interpolant}", "interpolantinterpolant")
}
#[test]
fn t41() {
    check("#{interpolant}+lschema_#{ritlp}", "interpolantlschema_ritlp")
}
#[test]
#[ignore]
fn t42() {
    check("#{interpolant} +lschema_#{ritlp}", "interpolantlschema_ritlp")
}
#[test]
fn t43() {
    check("#{interpolant}+ lschema_#{ritlp}", "interpolantlschema_ritlp")
}
#[test]
fn t44() {
    check("#{interpolant} + lschema_#{ritlp}", "interpolantlschema_ritlp")
}
#[test]
fn t45() {
    check("#{interpolant}+#{litlp}_rschema", "interpolantlitlp_rschema")
}
#[test]
#[ignore]
fn t46() {
    check("#{interpolant} +#{litlp}_rschema", "interpolantlitlp_rschema")
}
#[test]
fn t47() {
    check("#{interpolant}+ #{litlp}_rschema", "interpolantlitlp_rschema")
}
#[test]
fn t48() {
    check("#{interpolant} + #{litlp}_rschema", "interpolantlitlp_rschema")
}
#[test]
fn t49() {
    check("lschema_#{ritlp}+lschema_#{ritlp}", "lschema_ritlplschema_ritlp")
}
#[test]
#[ignore]
fn t50() {
    check("lschema_#{ritlp} +lschema_#{ritlp}", "lschema_ritlplschema_ritlp")
}
#[test]
fn t51() {
    check("lschema_#{ritlp}+ lschema_#{ritlp}", "lschema_ritlplschema_ritlp")
}
#[test]
fn t52() {
    check("lschema_#{ritlp} + lschema_#{ritlp}", "lschema_ritlplschema_ritlp")
}
#[test]
fn t53() {
    check("lschema_#{ritlp}+#{litlp}_rschema", "lschema_ritlplitlp_rschema")
}
#[test]
#[ignore]
fn t54() {
    check("lschema_#{ritlp} +#{litlp}_rschema", "lschema_ritlplitlp_rschema")
}
#[test]
fn t55() {
    check("lschema_#{ritlp}+ #{litlp}_rschema", "lschema_ritlplitlp_rschema")
}
#[test]
fn t56() {
    check("lschema_#{ritlp} + #{litlp}_rschema", "lschema_ritlplitlp_rschema")
}
#[test]
fn t57() {
    check("#{litlp}_rschema+#{litlp}_rschema", "litlp_rschemalitlp_rschema")
}
#[test]
#[ignore]
fn t58() {
    check("#{litlp}_rschema +#{litlp}_rschema", "litlp_rschemalitlp_rschema")
}
#[test]
fn t59() {
    check("#{litlp}_rschema+ #{litlp}_rschema", "litlp_rschemalitlp_rschema")
}
#[test]
fn t60() {
    check("#{litlp}_rschema + #{litlp}_rschema", "litlp_rschemalitlp_rschema")
}

fn check(expression: &str, expected: &str) {
    let mut scope = GlobalScope::new();
    let expression = format!("{};", expression);
    let (end, foo) = value_expression(expression.as_bytes()).unwrap();
    println!("Expression is: {:?}", foo);
    assert_eq!(Ok(";"), from_utf8(end));
    let result = foo.evaluate(&mut scope);
    println!(" ... evals to: {:?}", result);
    assert_eq!(format!("{}", result),
               expected)
}
