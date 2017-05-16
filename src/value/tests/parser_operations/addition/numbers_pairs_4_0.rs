//! Tests from
//! `sass-spec/spec/parser/operations/addition/numbers/pairs-4.0`.

use super::super::super::super::value_expression;
use std::str::from_utf8;
use variablescope::GlobalScope;

#[test]
fn t01() {
    check("10+10", "20")
}
#[test]
fn t02() {
    check("10 +10", "10 +10")
}
#[test]
fn t03() {
    check("10+ 10", "20")
}
#[test]
fn t04() {
    check("10 + 10", "20")
}
#[test]
fn t05() {
    check("10+#{10}", "1010")
}
#[test]
#[ignore]
fn t06() {
    check("10 +#{10}", "1010")
}
#[test]
fn t07() {
    check("10+ #{10}", "1010")
}
#[test]
fn t08() {
    check("10 + #{10}", "1010")
}
#[test]
fn t09() {
    check("10+1#{0}", "11 0")
}
#[test]
#[ignore]
fn t10() {
    check("10 +1#{0}", "11 0")
}
#[test]
fn t11() {
    check("10+ 1#{0}", "11 0")
}
#[test]
fn t12() {
    check("10 + 1#{0}", "11 0")
}
#[test]
fn t13() {
    check("10+#{1}0", "1010")
}
#[test]
#[ignore]
fn t14() {
    check("10 +#{1}0", "1010")
}
#[test]
fn t15() {
    check("10+ #{1}0", "1010")
}
#[test]
fn t16() {
    check("10 + #{1}0", "1010")
}
#[test]
fn t17() {
    check("#{10}+#{10}", "1010")
}
#[test]
#[ignore]
fn t18() {
    check("#{10} +#{10}", "1010")
}
#[test]
fn t19() {
    check("#{10}+ #{10}", "1010")
}
#[test]
fn t20() {
    check("#{10} + #{10}", "1010")
}
#[test]
fn t21() {
    check("#{10}+1#{0}", "101 0")
}
#[test]
#[ignore]
fn t22() {
    check("#{10} +1#{0}", "101 0")
}
#[test]
fn t23() {
    check("#{10}+ 1#{0}", "101 0")
}
#[test]
fn t24() {
    check("#{10} + 1#{0}", "101 0")
}
#[test]
fn t25() {
    check("#{10}+#{1}0", "1010")
}
#[test]
#[ignore]
fn t26() {
    check("#{10} +#{1}0", "1010")
}
#[test]
fn t27() {
    check("#{10}+ #{1}0", "1010")
}
#[test]
fn t28() {
    check("#{10} + #{1}0", "1010")
}
#[test]
#[ignore]
fn t29() {
    check("1#{0}+1#{0}", "1 01 0")
}
#[test]
#[ignore]
fn t30() {
    check("1#{0} +1#{0}", "1 01 0")
}
#[test]
#[ignore]
fn t31() {
    check("1#{0}+ 1#{0}", "1 01 0")
}
#[test]
#[ignore]
fn t32() {
    check("1#{0} + 1#{0}", "1 01 0")
}
#[test]
#[ignore]
fn t33() {
    check("1#{0}+#{1}0", "1 010")
}
#[test]
#[ignore]
fn t34() {
    check("1#{0} +#{1}0", "1 010")
}
#[test]
#[ignore]
fn t35() {
    check("1#{0}+ #{1}0", "1 010")
}
#[test]
#[ignore]
fn t36() {
    check("1#{0} + #{1}0", "1 010")
}
#[test]
fn t37() {
    check("#{1}0+#{1}0", "1010")
}
#[test]
#[ignore]
fn t38() {
    check("#{1}0 +#{1}0", "1010")
}
#[test]
fn t39() {
    check("#{1}0+ #{1}0", "1010")
}
#[test]
fn t40() {
    check("#{1}0 + #{1}0", "1010")
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
