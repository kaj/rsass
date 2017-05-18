//! Tests from
//! `sass-spec/spec/parser/operations/binary_and_unary`.

use super::check_eval;

#[test]
fn minus_before_minus() {
    check_eval("- 1 - 2", "-3")
}
#[test]
fn minus_after_minus() {
    check_eval("1 - - 2", "3")
}
#[test]
fn plus_before_minus() {
    check_eval("+ 1 - 2", "-1")
}
#[test]
fn plus_after_minus() {
    check_eval("1 - + 2", "-1")
}
#[test]
fn not_before_plus() {
    check_eval("not 1 + 2", "false2")
}
#[test]
fn not_after_plus() {
    check_eval("1 + not 2", "1false")
}
