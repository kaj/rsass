//! A scope is something that contains variable values.

use num_rational::Rational;
use std::collections::BTreeMap;
use std::ops::Neg;
use super::MixinDeclaration;
use valueexpression::{Value, Unit};

pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: BTreeMap<String, Value>,
    mixins: BTreeMap<String, MixinDeclaration>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Scope {
            parent: None,
            variables: BTreeMap::new(),
            mixins: BTreeMap::new(),
        }
    }
    pub fn sub(parent: &'a Scope) -> Self {
        Scope {
            parent: Some(parent),
            variables: BTreeMap::new(),
            mixins: BTreeMap::new(),
        }
    }
    pub fn define(&mut self, name: &str, val: &Value) {
        let val = self.do_evaluate(val, true);
        self.variables.insert(name.to_string(), val);
    }
    pub fn define_mixin(&mut self, m: &MixinDeclaration) {
        self.mixins.insert(m.name.to_string(), m.clone());
    }
    pub fn get_mixin(&self, name: &str) -> Option<&MixinDeclaration> {
        self.mixins
            .get(name)
            .or_else(|| self.parent.and_then(|p| p.get_mixin(name)))
    }
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables
            .get(name)
            .or_else(|| self.parent.and_then(|p| p.get(name)))
    }
    pub fn evaluate(&self, val: &Value) -> Value {
        self.do_evaluate(val, false)
    }
    fn do_evaluate(&self, val: &Value, arithmetic: bool) -> Value {
        match val {
            &Value::Literal(ref v) => Value::Literal(v.clone()),
            &Value::Paren(ref v) => self.do_evaluate(v, true),
            &Value::HexColor(_, _, _, _) => val.clone(),
            &Value::Variable(ref name) => {
                self.get(&name)
                    .map(|n| self.do_evaluate(n, true))
                    .unwrap_or_else(|| Value::Literal(format!("${}", name)))
            }
            &Value::MultiSpace(ref v) => {
                Value::MultiSpace(v.iter()
                    .map(|v| self.do_evaluate(v, false))
                    .collect::<Vec<_>>())
            }
            &Value::MultiComma(ref v) => {
                Value::MultiComma(v.iter()
                    .map(|v| self.do_evaluate(v, false))
                    .collect::<Vec<_>>())
            }
            &Value::Call(ref name, ref arg) => {
                Value::Call(name.clone(), Box::new(self.evaluate(arg)))
            }
            &Value::Sum(ref a, ref b) => {
                let a = self.do_evaluate(a, true);
                let b = self.do_evaluate(b, true);
                match (&a, &b) {
                    (&Value::HexColor(ref r, ref g, ref b, _),
                     &Value::Numeric(ref n, ref u, _)) if u == &Unit::None => {
                        Value::HexColor(add(r, n), add(g, n), add(b, n), None)
                    }
                    (&Value::HexColor(ref ar, ref ag, ref ab, _),
                     &Value::HexColor(ref br, ref bg, ref bb, _)) => {
                        Value::HexColor(add8(ar, br),
                                        add8(ag, bg),
                                        add8(ab, bb),
                                        None)
                    }
                    (&Value::Numeric(ref a, ref au, _),
                     &Value::Numeric(ref b, ref bu, _)) => {
                        if au == bu || bu == &Unit::None {
                            Value::Numeric(a + b, au.clone(), true)
                        } else if au == &Unit::None {
                            Value::Numeric(a + b, bu.clone(), true)
                        } else {
                            Value::Literal(format!("{}{}", a, b))
                        }
                    }
                    (a, b) => Value::Literal(format!("{}{}", a, b)),
                }
            }
            &Value::Minus(ref a, ref b) => {
                let a = self.evaluate(a);
                let b = self.evaluate(b);
                match (&a, &b) {
                    (&Value::HexColor(ref r, ref g, ref b, _),
                     &Value::Numeric(ref n, ref u, _)) if u == &Unit::None => {
                        let n = n.neg();
                        Value::HexColor(add(r, &n),
                                        add(g, &n),
                                        add(b, &n),
                                        None)
                    }
                    (&Value::HexColor(ref ar, ref ag, ref ab, _),
                     &Value::HexColor(ref br, ref bg, ref bb, _)) => {
                        Value::HexColor(sub8(ar, br),
                                        sub8(ag, bg),
                                        sub8(ab, bb),
                                        None)
                    }
                    (&Value::Numeric(ref a, ref au, _),
                     &Value::Numeric(ref b, ref bu, _)) => {
                        if au == bu {
                            Value::Numeric(a - b, au.clone(), true)
                        } else {
                            Value::Literal(format!("{}-{}", a, b))
                        }
                    }
                    (a, b) => Value::Literal(format!("{}-{}", a, b)),
                }
            }
            &Value::Product(ref a, ref b) => {
                let a = self.do_evaluate(a, true);
                let b = self.do_evaluate(b, true);
                if let (&Value::Numeric(ref a, ref au, _),
                        &Value::Numeric(ref b, ref bu, _)) = (&a, &b) {
                    if bu == &Unit::None {
                        Value::Numeric(a * b, au.clone(), true)
                    } else if au == &Unit::None {
                        Value::Numeric(a * b, bu.clone(), true)
                    } else {
                        Value::Literal(format!("{}*{}", a, b))
                    }
                } else {
                    Value::Literal(format!("{}*{}", a, b))
                }
            }
            &Value::Div(ref a, ref b, ref space1, ref space2) => {
                let (a, b) = {
                    let aa = self.do_evaluate(a, arithmetic);
                    let b =
                        self.do_evaluate(b, arithmetic || a.is_calculated());
                    if !arithmetic && b.is_calculated() && !a.is_calculated() {
                        (self.do_evaluate(a, true), b)
                    } else {
                        (aa, b)
                    }
                };
                if arithmetic || a.is_calculated() || b.is_calculated() {
                    match (&a, &b) {
                        (&Value::HexColor(ref r, ref g, ref b, _),
                         &Value::Numeric(ref n, Unit::None, _)) => {
                            return Value::HexColor(div(r, n),
                                                   div(g, n),
                                                   div(b, n),
                                                   None);
                        }
                        (&Value::Numeric(ref a, ref au, _),
                         &Value::Numeric(ref b, ref bu, _)) => {
                            if bu == &Unit::None {
                                return Value::Numeric(a / b, au.clone(), true);
                            } else if au == bu {
                                return Value::Numeric(a / b, Unit::None, true);
                            }
                        }
                        _ => (),
                    }
                }
                Value::Literal(format!("{}{}/{}{}",
                                       a,
                                       if *space1 && !arithmetic {
                                           " "
                                       } else {
                                           ""
                                       },
                                       if *space2 && !arithmetic {
                                           " "
                                       } else {
                                           ""
                                       },
                                       b))
            }
            &Value::Numeric(ref v, ref u, ref is_calculated) => {
                Value::Numeric(v.clone(),
                               u.clone(),
                               arithmetic || *is_calculated)
            }
        }
    }
}

fn add(x: &u8, y: &Rational) -> u8 {
    let v = *x as f32 + *y.numer() as f32 / *y.denom() as f32;
    if v < 0.0 {
        0
    } else if v > 255.0 {
        0xff
    } else {
        v as u8
    }
}
fn div(x: &u8, y: &Rational) -> u8 {
    let v = *x as f32 * *y.denom() as f32 / *y.numer() as f32;
    if v < 0.0 {
        0
    } else if v > 255.0 {
        0xff
    } else {
        v as u8
    }
}

fn add8(x: &u8, y: &u8) -> u8 {
    match x.overflowing_add(*y) {
        (_, true) => 0xff,
        (s, false) => s,
    }
}

fn sub8(x: &u8, y: &u8) -> u8 {
    if *x > *y { *x - *y } else { 0 }
}

#[cfg(test)]
mod test {
    use num_rational::Rational;
    use valueexpression::*;
    use variablescope::*;

    #[test]
    fn variable_value() {
        let mut scope = Scope::new();
        let red = Value::Literal("#f02a42".to_string());
        scope.define("red", &red);
        assert_eq!("#f02a42", do_evaluate(&scope, b"$red;"));
    }

    #[test]
    fn partial_variable_value() {
        let mut scope = Scope::new();
        let red = Value::Literal("#f02a42".to_string());
        scope.define("red", &red);
        assert_eq!("solid 1px #f02a42",
                   do_evaluate(&scope, b"solid 1px $red;"));
    }

    #[test]
    fn simple_arithmetic() {
        let scope = Scope::new();
        assert_eq!("6", do_evaluate(&scope, b"3 + 3;"));
    }

    #[test]
    fn simple_arithmetic_2() {
        let scope = Scope::new();
        assert_eq!("14", do_evaluate(&scope, b"2 + 3 * 4;"));
    }

    #[test]
    fn simple_arithmetic_3() {
        let mut scope = Scope::new();
        let four = Value::Numeric(Rational::from_integer(4), Unit::None, false);
        scope.define("four", &four);
        assert_eq!("14", do_evaluate(&scope, b"2 + 3 * $four;"));
    }

    // The following tests are from aboud division are from
    // http://sass-lang.com/documentation/file.SASS_REFERENCE.html ,
    // Section "Divison and /"
    #[test]
    fn div_slash_1() {
        let scope = Scope::new();
        assert_eq!("10px/8px", do_evaluate(&scope, b"10px/8px;"));
    }

    #[test]
    fn div_slash_2() {
        let mut scope = Scope::new();
        let width =
            Value::Numeric(Rational::from_integer(1000), Unit::Px, false);
        scope.define("width", &width);
        assert_eq!("500px", do_evaluate(&scope, b"$width/2;"));
    }

    #[test]
    fn div_slash_4() {
        let scope = Scope::new();
        assert_eq!("250px", do_evaluate(&scope, b"(500px/2);"));
    }

    #[test]
    fn div_slash_5() {
        let scope = Scope::new();
        assert_eq!("9px", do_evaluate(&scope, b"5px + 8px/2px;"));
    }

    #[test]
    fn div_slash_6() {
        let scope = Scope::new();
        assert_eq!("italic bold 10px/8px",
                   do_evaluate(&scope, b"(italic bold 10px/8px);"));
    }


    // ...
    #[test]
    fn double_div_1() {
        let scope = Scope::new();
        assert_eq!("15/3/5", do_evaluate(&scope, b"15/3/5;"));
    }

    #[test]
    fn double_div_2() {
        let scope = Scope::new();
        assert_eq!("15 / 3 / 5", do_evaluate(&scope, b"15 / 3 / 5;"));
    }

    #[test]
    fn double_div_3() {
        let scope = Scope::new();
        assert_eq!("1", do_evaluate(&scope, b"(15 / 3 / 5);"));
    }

    #[test]
    fn double_div_4() {
        let scope = Scope::new();
        assert_eq!("1", do_evaluate(&scope, b"(15 / 3) / 5;"));
    }

    #[test]
    fn double_div_5() {
        let mut scope = Scope::new();
        let five = do_parse(b"5;");
        scope.define("five", &five);
        assert_eq!("1", do_evaluate(&scope, b"15 / 3 / $five;"));
    }

    #[test]
    fn sum_w_unit() {
        let scope = Scope::new();
        assert_eq!("9px", do_evaluate(&scope, b"3px + 3px + 3px;"));
    }
    #[test]
    fn multi_multi() {
        let mut scope = Scope::new();
        let stuff = do_parse(b"1 2 3;");
        scope.define("stuff", &stuff);
        assert_eq!("1 2 3, 1 2 3 4 5 6, 7 8 9",
                   do_evaluate(&scope, b"1 2 3, $stuff 4 5 (6, 7 8 9);"))
    }

    #[test]
    fn url_keeps_parens() {
        let scope = Scope::new();
        assert_eq!("black url(starfield.png) repeat",
                   do_evaluate(&scope, b"black url(starfield.png) repeat;"))
    }

    #[test]
    fn color_unchanged_1() {
        let scope = Scope::new();
        assert_eq!("#AbC", do_evaluate(&scope, b"#AbC;"))
    }

    #[test]
    fn color_unchanged_2() {
        let scope = Scope::new();
        assert_eq!("#AAbbCC", do_evaluate(&scope, b"#AAbbCC;"))
    }

    #[test]
    fn color_add_each_component() {
        let scope = Scope::new();
        assert_eq!("#abbccd", do_evaluate(&scope, b"#AbC + 1;"))
    }
    #[test]
    fn color_add_each_component_overflow() {
        let scope = Scope::new();
        assert_eq!("#0101ff", do_evaluate(&scope, b"#00f + 1;"))
    }

    #[test]
    fn color_add_components() {
        let scope = Scope::new();
        assert_eq!("#aabbdd", do_evaluate(&scope, b"#AbC + #001;"))
    }

    #[test]
    fn color_add_components_overflow() {
        let scope = Scope::new();
        assert_eq!("#1000ff", do_evaluate(&scope, b"#1000ff + #001;"))
    }

    #[test]
    fn color_add_components_to_named_overflow() {
        let scope = Scope::new();
        assert_eq!("blue", do_evaluate(&scope, b"#0000ff + #001;"))
    }
    #[test]
    fn color_add_components_to_named() {
        let scope = Scope::new();
        assert_eq!("white", do_evaluate(&scope, b"#00f + #0f0 + #f00;"))
    }

    #[test]
    fn color_subtract() {
        let scope = Scope::new();
        assert_eq!("#fefefe", do_evaluate(&scope, b"#fff - 1;"))
    }

    #[test]
    fn color_subtract_underflow() {
        let scope = Scope::new();
        assert_eq!("black", do_evaluate(&scope, b"#000 - 1;"))
    }

    #[test]
    fn color_subtract_components() {
        let scope = Scope::new();
        assert_eq!("#000077", // Or should it be #007?
                   do_evaluate(&scope, b"#fff - #ff8;"))
    }

    #[test]
    fn color_subtract_components_underflow() {
        let scope = Scope::new();
        assert_eq!("black", do_evaluate(&scope, b"#000001 - #001;"))
    }

    #[test]
    fn color_division() {
        let scope = Scope::new();
        assert_eq!("#020202", do_evaluate(&scope, b"(#101010 / 7);"))
    }

    #[test]
    fn color_add_rgb_1() {
        let scope = Scope::new();
        assert_eq!("#0b0a0b", do_evaluate(&scope, b"rgb(10,10,10) + #010001;"))
    }
    #[test]
    fn color_add_rgb_2() {
        let scope = Scope::new();
        assert_eq!("white",
                   do_evaluate(&scope, b"#010000 + rgb(255, 255, 255);"))
    }

    fn do_evaluate(scope: &Scope, expression: &[u8]) -> String {
        let (end, foo) = value_expression(expression).unwrap();
        assert_eq!(b";", end);
        format!("{}", scope.evaluate(&foo))
    }

    fn do_parse(expression: &[u8]) -> Value {
        let (end, foo) = value_expression(expression).unwrap();
        assert_eq!(b";", end);
        foo
    }
}
