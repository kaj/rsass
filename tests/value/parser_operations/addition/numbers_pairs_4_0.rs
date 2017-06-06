//! Tests from
//! `sass-spec/spec/parser/operations/addition/numbers/pairs-4.0`.
//!
//! All tests marked `ignore` here are TODO.

use super::super::check_eval;

#[test]
fn t01() {
    check_eval("10+10", "20")
}
#[test]
fn t02() {
    check_eval("10 +10", "10 +10")
}
#[test]
fn t03() {
    check_eval("10+ 10", "20")
}
#[test]
fn t04() {
    check_eval("10 + 10", "20")
}
#[test]
fn t05() {
    check_eval("10+#{10}", "1010")
}
#[test]
#[ignore]
fn t06() {
    check_eval("10 +#{10}", "1010")
}
#[test]
fn t07() {
    check_eval("10+ #{10}", "1010")
}
#[test]
fn t08() {
    check_eval("10 + #{10}", "1010")
}
#[test]
fn t09() {
    check_eval("10+1#{0}", "11 0")
}
#[test]
#[ignore]
fn t10() {
    check_eval("10 +1#{0}", "11 0")
}
#[test]
fn t11() {
    check_eval("10+ 1#{0}", "11 0")
}
#[test]
fn t12() {
    check_eval("10 + 1#{0}", "11 0")
}
#[test]
fn t13() {
    check_eval("10+#{1}0", "1010")
}
#[test]
#[ignore]
fn t14() {
    check_eval("10 +#{1}0", "1010")
}
#[test]
fn t15() {
    check_eval("10+ #{1}0", "1010")
}
#[test]
fn t16() {
    check_eval("10 + #{1}0", "1010")
}
#[test]
fn t17() {
    check_eval("#{10}+#{10}", "1010")
}
#[test]
#[ignore]
fn t18() {
    check_eval("#{10} +#{10}", "1010")
}
#[test]
fn t19() {
    check_eval("#{10}+ #{10}", "1010")
}
#[test]
fn t20() {
    check_eval("#{10} + #{10}", "1010")
}
#[test]
fn t21() {
    check_eval("#{10}+1#{0}", "101 0")
}
#[test]
#[ignore]
fn t22() {
    check_eval("#{10} +1#{0}", "101 0")
}
#[test]
fn t23() {
    check_eval("#{10}+ 1#{0}", "101 0")
}
#[test]
fn t24() {
    check_eval("#{10} + 1#{0}", "101 0")
}
#[test]
fn t25() {
    check_eval("#{10}+#{1}0", "1010")
}
#[test]
#[ignore]
fn t26() {
    check_eval("#{10} +#{1}0", "1010")
}
#[test]
fn t27() {
    check_eval("#{10}+ #{1}0", "1010")
}
#[test]
fn t28() {
    check_eval("#{10} + #{1}0", "1010")
}
#[test]
#[ignore]
fn t29() {
    check_eval("1#{0}+1#{0}", "1 01 0")
}
#[test]
#[ignore]
fn t30() {
    check_eval("1#{0} +1#{0}", "1 01 0")
}
#[test]
#[ignore]
fn t31() {
    check_eval("1#{0}+ 1#{0}", "1 01 0")
}
#[test]
#[ignore]
fn t32() {
    check_eval("1#{0} + 1#{0}", "1 01 0")
}
#[test]
#[ignore]
fn t33() {
    check_eval("1#{0}+#{1}0", "1 010")
}
#[test]
#[ignore]
fn t34() {
    check_eval("1#{0} +#{1}0", "1 010")
}
#[test]
#[ignore]
fn t35() {
    check_eval("1#{0}+ #{1}0", "1 010")
}
#[test]
#[ignore]
fn t36() {
    check_eval("1#{0} + #{1}0", "1 010")
}
#[test]
fn t37() {
    check_eval("#{1}0+#{1}0", "1010")
}
#[test]
#[ignore]
fn t38() {
    check_eval("#{1}0 +#{1}0", "1010")
}
#[test]
fn t39() {
    check_eval("#{1}0+ #{1}0", "1010")
}
#[test]
fn t40() {
    check_eval("#{1}0 + #{1}0", "1010")
}
