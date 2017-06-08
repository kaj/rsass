//! Tests from
//! `sass-spec/spec/parser/operations/addition/strings/pairs-4.0`.
//!
//! All tests marked `ignore` here are TODO.

use super::super::check_eval;

#[test]
fn t01() {
    check_eval("literal+literal", "literalliteral")
}
#[test]
#[ignore]
fn t02() {
    check_eval("literal +literal", "literalliteral")
}
#[test]
fn t03() {
    check_eval("literal+ literal", "literalliteral")
}
#[test]
fn t04() {
    check_eval("literal + literal", "literalliteral")
}
#[test]
fn t05() {
    check_eval("literal+\"quoted\"", "literalquoted")
}
#[test]
#[ignore]
fn t06() {
    check_eval("literal +\"quoted\"", "literalquoted")
}
#[test]
fn t07() {
    check_eval("literal+ \"quoted\"", "literalquoted")
}
#[test]
fn t08() {
    check_eval("literal + \"quoted\"", "literalquoted")
}
#[test]
fn t09() {
    check_eval("literal+#{interpolant}", "literalinterpolant")
}
#[test]
#[ignore]
fn t10() {
    check_eval("literal +#{interpolant}", "literalinterpolant")
}
#[test]
fn t11() {
    check_eval("literal+ #{interpolant}", "literalinterpolant")
}
#[test]
fn t12() {
    check_eval("literal + #{interpolant}", "literalinterpolant")
}
#[test]
fn t13() {
    check_eval("literal+lschema_#{ritlp}", "literallschema_ritlp")
}
#[test]
#[ignore]
fn t14() {
    check_eval("literal +lschema_#{ritlp}", "literallschema_ritlp")
}
#[test]
fn t15() {
    check_eval("literal+ lschema_#{ritlp}", "literallschema_ritlp")
}
#[test]
fn t16() {
    check_eval("literal + lschema_#{ritlp}", "literallschema_ritlp")
}
#[test]
fn t17() {
    check_eval("literal+#{litlp}_rschema", "literallitlp_rschema")
}
#[test]
#[ignore]
fn t18() {
    check_eval("literal +#{litlp}_rschema", "literallitlp_rschema")
}
#[test]
fn t19() {
    check_eval("literal+ #{litlp}_rschema", "literallitlp_rschema")
}
#[test]
fn t20() {
    check_eval("literal + #{litlp}_rschema", "literallitlp_rschema")
}
#[test]
fn t21() {
    check_eval("\"quoted\"+\"quoted\"", "\"quotedquoted\"")
}
#[test]
#[ignore]
fn t22() {
    check_eval("\"quoted\" +\"quoted\"", "\"quotedquoted\"")
}
#[test]
fn t23() {
    check_eval("\"quoted\"+ \"quoted\"", "\"quotedquoted\"")
}
#[test]
fn t24() {
    check_eval("\"quoted\" + \"quoted\"", "\"quotedquoted\"")
}
#[test]
fn t25() {
    check_eval("\"quoted\"+#{interpolant}", "\"quotedinterpolant\"")
}
#[test]
#[ignore]
fn t26() {
    check_eval("\"quoted\" +#{interpolant}", "\"quotedinterpolant\"")
}
#[test]
fn t27() {
    check_eval("\"quoted\"+ #{interpolant}", "\"quotedinterpolant\"")
}
#[test]
fn t28() {
    check_eval("\"quoted\" + #{interpolant}", "\"quotedinterpolant\"")
}
#[test]
fn t29() {
    check_eval("\"quoted\"+lschema_#{ritlp}", "\"quotedlschema_ritlp\"")
}
#[test]
#[ignore]
fn t30() {
    check_eval("\"quoted\" +lschema_#{ritlp}", "\"quotedlschema_ritlp\"")
}
#[test]
fn t31() {
    check_eval("\"quoted\"+ lschema_#{ritlp}", "\"quotedlschema_ritlp\"")
}
#[test]
fn t32() {
    check_eval("\"quoted\" + lschema_#{ritlp}", "\"quotedlschema_ritlp\"")
}
#[test]
fn t33() {
    check_eval("\"quoted\"+#{litlp}_rschema", "\"quotedlitlp_rschema\"")
}
#[test]
#[ignore]
fn t34() {
    check_eval("\"quoted\" +#{litlp}_rschema", "\"quotedlitlp_rschema\"")
}
#[test]
fn t35() {
    check_eval("\"quoted\"+ #{litlp}_rschema", "\"quotedlitlp_rschema\"")
}
#[test]
fn t36() {
    check_eval("\"quoted\" + #{litlp}_rschema", "\"quotedlitlp_rschema\"")
}
#[test]
fn t37() {
    check_eval("#{interpolant}+#{interpolant}", "interpolantinterpolant")
}
#[test]
#[ignore]
fn t38() {
    check_eval("#{interpolant} +#{interpolant}", "interpolantinterpolant")
}
#[test]
fn t39() {
    check_eval("#{interpolant}+ #{interpolant}", "interpolantinterpolant")
}
#[test]
fn t40() {
    check_eval("#{interpolant} + #{interpolant}", "interpolantinterpolant")
}
#[test]
fn t41() {
    check_eval("#{interpolant}+lschema_#{ritlp}", "interpolantlschema_ritlp")
}
#[test]
#[ignore]
fn t42() {
    check_eval("#{interpolant} +lschema_#{ritlp}", "interpolantlschema_ritlp")
}
#[test]
fn t43() {
    check_eval("#{interpolant}+ lschema_#{ritlp}", "interpolantlschema_ritlp")
}
#[test]
fn t44() {
    check_eval("#{interpolant} + lschema_#{ritlp}", "interpolantlschema_ritlp")
}
#[test]
fn t45() {
    check_eval("#{interpolant}+#{litlp}_rschema", "interpolantlitlp_rschema")
}
#[test]
#[ignore]
fn t46() {
    check_eval("#{interpolant} +#{litlp}_rschema", "interpolantlitlp_rschema")
}
#[test]
fn t47() {
    check_eval("#{interpolant}+ #{litlp}_rschema", "interpolantlitlp_rschema")
}
#[test]
fn t48() {
    check_eval("#{interpolant} + #{litlp}_rschema", "interpolantlitlp_rschema")
}
#[test]
fn t49() {
    check_eval("lschema_#{ritlp}+lschema_#{ritlp}",
               "lschema_ritlplschema_ritlp")
}
#[test]
#[ignore]
fn t50() {
    check_eval("lschema_#{ritlp} +lschema_#{ritlp}",
               "lschema_ritlplschema_ritlp")
}
#[test]
fn t51() {
    check_eval("lschema_#{ritlp}+ lschema_#{ritlp}",
               "lschema_ritlplschema_ritlp")
}
#[test]
fn t52() {
    check_eval("lschema_#{ritlp} + lschema_#{ritlp}",
               "lschema_ritlplschema_ritlp")
}
#[test]
fn t53() {
    check_eval("lschema_#{ritlp}+#{litlp}_rschema",
               "lschema_ritlplitlp_rschema")
}
#[test]
#[ignore]
fn t54() {
    check_eval("lschema_#{ritlp} +#{litlp}_rschema",
               "lschema_ritlplitlp_rschema")
}
#[test]
fn t55() {
    check_eval("lschema_#{ritlp}+ #{litlp}_rschema",
               "lschema_ritlplitlp_rschema")
}
#[test]
fn t56() {
    check_eval("lschema_#{ritlp} + #{litlp}_rschema",
               "lschema_ritlplitlp_rschema")
}
#[test]
fn t57() {
    check_eval("#{litlp}_rschema+#{litlp}_rschema",
               "litlp_rschemalitlp_rschema")
}
#[test]
#[ignore]
fn t58() {
    check_eval("#{litlp}_rschema +#{litlp}_rschema",
               "litlp_rschemalitlp_rschema")
}
#[test]
fn t59() {
    check_eval("#{litlp}_rschema+ #{litlp}_rschema",
               "litlp_rschemalitlp_rschema")
}
#[test]
fn t60() {
    check_eval("#{litlp}_rschema + #{litlp}_rschema",
               "litlp_rschemalitlp_rschema")
}
