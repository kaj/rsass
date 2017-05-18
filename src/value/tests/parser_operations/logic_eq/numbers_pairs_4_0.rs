//! Tests from
//! `spec/parser/operations/logic_eq/numbers/pairs-4.0`.

use super::super::check_eval;

#[test]
fn t01() {
    check_eval("10==10", "true")
}
#[test]
fn t02() {
    check_eval("10 ==10", "true")
}
#[test]
fn t03() {
    check_eval("10== 10", "true")
}
#[test]
fn t04() {
    check_eval("10 == 10", "true")
}
#[test]
fn t05() {
    check_eval("10==#{10}", "false")
}
#[test]
fn t06() {
    check_eval("10 ==#{10}", "false")
}
#[test]
fn t07() {
    check_eval("10== #{10}", "false")
}
#[test]
fn t08() {
    check_eval("10 == #{10}", "false")
}
#[test]
fn t09() {
    check_eval("10==1#{0}", "false 0")
}
#[test]
fn t10() {
    check_eval("10 ==1#{0}", "false 0")
}
#[test]
fn t11() {
    check_eval("10== 1#{0}", "false 0")
}
#[test]
fn t12() {
    check_eval("10 == 1#{0}", "false 0")
}
#[test]
fn t13() {
    check_eval("10==#{1}0", "false")
}
#[test]
fn t14() {
    check_eval("10 ==#{1}0", "false")
}
#[test]
fn t15() {
    check_eval("10== #{1}0", "false")
}
#[test]
fn t16() {
    check_eval("10 == #{1}0", "false")
}
#[test]
fn t17() {
    check_eval("#{10}==#{10}", "true")
}
#[test]
fn t18() {
    check_eval("#{10} ==#{10}", "true")
}
#[test]
fn t19() {
    check_eval("#{10}== #{10}", "true")
}
#[test]
fn t20() {
    check_eval("#{10} == #{10}", "true")
}
#[test]
fn t21() {
    check_eval("#{10}==1#{0}", "false 0")
}
#[test]
fn t22() {
    check_eval("#{10} ==1#{0}", "false 0")
}
#[test]
fn t23() {
    check_eval("#{10}== 1#{0}", "false 0")
}
#[test]
fn t24() {
    check_eval("#{10} == 1#{0}", "false 0")
}
#[test]
fn t25() {
    check_eval("#{10}==#{1}0", "true")
}
#[test]
fn t26() {
    check_eval("#{10} ==#{1}0", "true")
}
#[test]
fn t27() {
    check_eval("#{10}== #{1}0", "true")
}
#[test]
fn t28() {
    check_eval("#{10} == #{1}0", "true")
}
#[test]
fn t29() {
    check_eval("1#{0}==1#{0}", "1 false 0")
}
#[test]
fn t30() {
    check_eval("1#{0} ==1#{0}", "1 false 0")
}
#[test]
fn t31() {
    check_eval("1#{0}== 1#{0}", "1 false 0")
}
#[test]
fn t32() {
    check_eval("1#{0} == 1#{0}", "1 false 0")
}
#[test]
fn t33() {
    check_eval("1#{0}==#{1}0", "1 false")
}
#[test]
fn t34() {
    check_eval("1#{0} ==#{1}0", "1 false")
}
#[test]
fn t35() {
    check_eval("1#{0}== #{1}0", "1 false")
}
#[test]
fn t36() {
    check_eval("1#{0} == #{1}0", "1 false")
}
#[test]
fn t37() {
    check_eval("#{1}0==#{1}0", "true")
}
#[test]
fn t38() {
    check_eval("#{1}0 ==#{1}0", "true")
}
#[test]
fn t39() {
    check_eval("#{1}0== #{1}0", "true")
}
#[test]
fn t40() {
    check_eval("#{1}0 == #{1}0", "true")
}
