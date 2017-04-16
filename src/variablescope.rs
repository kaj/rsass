//! A scope is something that contains variable values.

use super::SassItem;
use formalargs::{CallArgs, FormalArgs};
use functions::{SassFunction, get_builtin_function};
use num_traits::identities::Zero;
use std::collections::BTreeMap;
use unit::Unit;
use valueexpression::{Quotes, Value};

pub struct ScopeImpl<'a> {
    parent: Option<&'a mut Scope>,
    variables: BTreeMap<String, Value>,
    mixins: BTreeMap<String, (FormalArgs, Vec<SassItem>)>,
    functions: BTreeMap<String, SassFunction>,
}

pub trait Scope {
    fn define(&mut self, name: &str, val: &Value, global: bool);
    fn define_default(&mut self, name: &str, val: &Value, global: bool);
    fn get(&self, name: &str) -> Value;
    fn get_global(&self, name: &str) -> Value;

    fn define_mixin(&mut self,
                    name: &str,
                    args: &FormalArgs,
                    body: &[SassItem]);
    fn get_mixin(&self, name: &str) -> Option<(FormalArgs, Vec<SassItem>)>;

    fn define_function(&mut self, name: &str, func: SassFunction);
    fn get_function(&self, name: &str) -> Option<&SassFunction>;
    fn call_function(&mut self, name: &str, args: &CallArgs) -> Option<Value>;

    fn evaluate(&mut self, val: &Value) -> Value;

    fn eval_body(&mut self, body: &[SassItem]) -> Option<Value>;
}

impl<'a> Scope for ScopeImpl<'a> {
    fn define(&mut self, name: &str, val: &Value, global: bool) {
        let val = self.do_evaluate(val, true);
        if let (true, Some(parent)) = (global, self.parent.as_mut()) {
            return parent.define(name, &val, global);
        }
        self.variables.insert(name.replace('-', "_"), val);
    }
    fn define_default(&mut self, name: &str, val: &Value, global: bool) {
        if self.get(name) == Value::Null {
            self.define(name, val, global)
        }
    }
    fn get_mixin(&self, name: &str) -> Option<(FormalArgs, Vec<SassItem>)> {
        self.mixins
            .get(&name.replace('-', "_"))
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_mixin(name)))
    }
    fn get(&self, name: &str) -> Value {
        let name = name.replace('-', "_");
        self.variables
            .get(&name)
            .cloned()
            .or_else(|| self.parent.as_ref().map(|p| p.get(&name)))
            .unwrap_or(Value::Null)
    }
    fn get_global(&self, name: &str) -> Value {
        if let Some(ref p) = self.parent {
            p.get_global(name)
        } else {
            self.variables.get(name).cloned().unwrap_or(Value::Null)
        }
    }
    fn define_mixin(&mut self,
                    name: &str,
                    args: &FormalArgs,
                    body: &[SassItem]) {
        self.mixins.insert(name.replace('-', "_"), (args.clone(), body.into()));
    }
    fn define_function(&mut self, name: &str, func: SassFunction) {
        self.functions.insert(name.replace('-', "_"), func);
    }
    fn get_function(&self, name: &str) -> Option<&SassFunction> {
        let name = name.replace('-', "_");
        if let Some(f) = self.functions.get(&name) {
            return Some(f);
        }
        if let Some(ref p) = self.parent {
            p.get_function(&name)
        } else {
            get_builtin_function(&name)
        }
    }
    fn call_function(&mut self, name: &str, args: &CallArgs) -> Option<Value> {
        let name = name.replace('-', "_");
        if let Some(f) = self.functions.get(&name).cloned() {
            return f.call(self, args).ok();
        }
        let a2 = args.xyzzy(self);
        if let Some(ref mut p) = self.parent {
            return p.call_function(&name, &a2);
        }
        None
    }
    fn evaluate(&mut self, val: &Value) -> Value {
        self.do_evaluate(val, false)
    }

    fn eval_body(&mut self, body: &[SassItem]) -> Option<Value> {
        for b in body {
            let result = match *b {
                SassItem::IfStatement(ref cond, ref do_if, ref do_else) => {
                    if self.evaluate(cond).is_true() {
                        self.eval_body(do_if)
                    } else {
                        self.eval_body(do_else)
                    }
                }
                SassItem::Each(ref name, ref values, ref body) => {
                    let values = match self.evaluate(values) {
                        Value::List(v, _) => v,
                        v => vec![v],
                    };
                    for value in values {
                        self.define(name, &value, false);
                        if let Some(r) = self.eval_body(body) {
                            return Some(r);
                        }
                    }
                    None
                }
                SassItem::For {
                    ref name,
                    ref from,
                    ref to,
                    inclusive,
                    ref body,
                } => {
                    let from = self.evaluate(from).integer_value().unwrap();
                    let to = self.evaluate(to).integer_value().unwrap();
                    let to = if inclusive { to + 1 } else { to };
                    for value in from..to {
                        self.define(name, &Value::scalar(value), false);
                        if let Some(r) = self.eval_body(body) {
                            return Some(r);
                        }
                    }
                    None
                }
                SassItem::VariableDeclaration {
                    ref name,
                    ref val,
                    default,
                    global,
                } => {
                    if default {
                        self.define_default(name, val, global);
                    } else {
                        self.define(name, val, global);
                    }
                    None
                }
                SassItem::Return(ref v) => Some(self.evaluate(v)),
                SassItem::While(ref cond, ref body) => {
                    let mut scope = ScopeImpl::sub(self);
                    while scope.evaluate(cond).is_true() {
                        if let Some(r) = scope.eval_body(body) {
                            return Some(r);
                        }
                    }
                    None
                }
                SassItem::None => None,
                ref x => {
                    panic!("Not implemented in fuction: {:?}", x);
                }
            };
            if let Some(result) = result {
                return Some(result);
            }
        }
        None
    }
}

impl<'a> ScopeImpl<'a> {
    pub fn new() -> Self {
        ScopeImpl {
            parent: None,
            variables: BTreeMap::new(),
            mixins: BTreeMap::new(),
            functions: BTreeMap::new(),
        }
    }
    pub fn sub(parent: &'a mut Scope) -> Self {
        ScopeImpl {
            parent: Some(parent),
            variables: BTreeMap::new(),
            mixins: BTreeMap::new(),
            functions: BTreeMap::new(),
        }
    }
    fn do_evaluate(&mut self, val: &Value, arithmetic: bool) -> Value {
        match val {
            &Value::Literal(ref v, ref q) => {
                Value::Literal(v.clone(), q.clone())
            }
            &Value::Paren(ref v) => self.do_evaluate(v, true),
            &Value::Color(_, _, _, _, _) => val.clone(),
            &Value::Variable(ref name) => {
                let v = self.get(name);
                self.do_evaluate(&v, true)
            }
            &Value::List(ref v, ref s) => {
                Value::List(v.iter()
                                .map(|v| self.do_evaluate(v, false))
                                .collect::<Vec<_>>(),
                            s.clone())
            }
            &Value::Call(ref name, ref args) => {
                match self.call_function(name, args) {
                    Some(value) => value,
                    None => {
                        if let Some(function) = get_builtin_function(name) {
                            match function.call(&mut *self, args) {
                                Ok(v) => v,
                                Err(e) => {
                                    panic!("Error in function {}: {:?}",
                                           name, e)
                                }
                            }
                        } else {
                            Value::Call(name.clone(), args.xyzzy(self))
                        }
                    }
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
                        (&Value::Color(ref r, ref g, ref b, ref a, _),
                         &Value::Numeric(ref n, Unit::None, _)) => {
                            return Value::rgba(r / n, g / n, b / n, *a);
                        }
                        (&Value::Numeric(ref av, ref au, _),
                         &Value::Numeric(ref bv, ref bu, _)) => {
                            if bv.is_zero() {
                                return Value::Div(Box::new(a.clone()),
                                                  Box::new(b.clone()),
                                                  *space1,
                                                  *space2);
                            } else if bu == &Unit::None {
                                return Value::Numeric(av / bv,
                                                      au.clone(),
                                                      true);
                            } else if au == bu {
                                return Value::Numeric(av / bv,
                                                      Unit::None,
                                                      true);
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
                                       b),
                               Quotes::None)
            }
            &Value::Numeric(ref v, ref u, ref is_calculated) => {
                Value::Numeric(*v, u.clone(), arithmetic || *is_calculated)
            }
            &Value::Null => Value::Null,
            &Value::True => Value::True,
            &Value::False => Value::False,
            &Value::BinOp(ref a, ref op, ref b) => {
                op.eval(self.do_evaluate(a, true), self.do_evaluate(b, true))
            }
            &Value::UnaryOp(ref op, ref v) => {
                Value::UnaryOp(op.clone(), Box::new(self.do_evaluate(v, true)))
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::str::from_utf8;
    use valueexpression::*;
    use variablescope::*;

    #[test]
    fn variable_value() {
        assert_eq!("#f02a42", do_evaluate(&[("red", "#f02a42")], b"$red;"))
    }

    #[test]
    fn partial_variable_value() {
        let scope = [("red", "#f02a42")];
        assert_eq!("solid 1px #f02a42", do_evaluate(&scope, b"solid 1px $red;"))
    }

    #[test]
    fn simple_arithmetic() {
        assert_eq!("6", do_evaluate(&[], b"3 + 3;"))
    }

    #[test]
    fn simple_arithmetic_2() {
        assert_eq!("14", do_evaluate(&[], b"2 + 3 * 4;"))
    }

    #[test]
    fn simple_arithmetic_3() {
        assert_eq!("14", do_evaluate(&[("four", "4")], b"2 + 3 * $four;"))
    }

    // The following tests are from aboud division are from
    // http://sass-lang.com/documentation/file.SASS_REFERENCE.html ,
    // Section "Divison and /"
    #[test]
    fn div_slash_1() {
        assert_eq!("10px/8px", do_evaluate(&[], b"10px/8px;"))
    }

    #[test]
    fn div_slash_2() {
        assert_eq!("500px", do_evaluate(&[("width", "1000px")], b"$width/2;"))
    }

    #[test]
    fn div_slash_4() {
        assert_eq!("250px", do_evaluate(&[], b"(500px/2);"))
    }

    #[test]
    fn div_slash_5() {
        assert_eq!("9px", do_evaluate(&[], b"5px + 8px/2px;"))
    }

    #[test]
    fn div_slash_6() {
        assert_eq!("italic bold 10px/8px",
                   do_evaluate(&[], b"(italic bold 10px/8px);"))
    }
    #[test]
    fn negative_in_arithmetic() {
        assert_eq!("960px", do_evaluate(&[("m", "20")], b"1000px + $m * -2;"))
    }

    // ...
    #[test]
    fn div_by_zero() {
        assert_eq!("500px/0", do_evaluate(&[], b"(500px/0);"))
    }

    #[test]
    fn double_div_1() {
        assert_eq!("15/3/5", do_evaluate(&[], b"15/3/5;"))
    }

    #[test]
    fn double_div_2() {
        assert_eq!("15 / 3 / 5", do_evaluate(&[], b"15 / 3 / 5;"))
    }

    #[test]
    fn double_div_3() {
        assert_eq!("1", do_evaluate(&[], b"(15 / 3 / 5);"))
    }

    #[test]
    fn long_div_and_mul_sequence() {
        assert_eq!("3", do_evaluate(&[], b"(3 / 2 / 2 / 2 * 32 / 2 / 2);"))
    }

    #[test]
    fn double_div_4() {
        assert_eq!("1", do_evaluate(&[], b"(15 / 3) / 5;"));
    }

    #[test]
    fn double_div_5() {
        assert_eq!("1", do_evaluate(&[("five", "5")], b"15 / 3 / $five;"))
    }

    #[test]
    fn sum_w_unit() {
        assert_eq!("9px", do_evaluate(&[], b"3px + 3px + 3px;"))
    }
    #[test]
    fn multi_multi() {
        let scope = [("stuff", "1 2 3")];
        assert_eq!("1 2 3, 1 2 3 4 5 6, 7 8 9",
                   do_evaluate(&scope, b"1 2 3, $stuff 4 5 (6, 7 8 9);"))
    }

    #[test]
    fn url_keeps_parens() {
        assert_eq!("black url(starfield.png) repeat",
                   do_evaluate(&[], b"black url(starfield.png) repeat;"))
    }

    #[test]
    fn color_unchanged_1() {
        assert_eq!("#AbC", do_evaluate(&[], b"#AbC;"))
    }

    #[test]
    fn color_unchanged_2() {
        assert_eq!("#AAbbCC", do_evaluate(&[], b"#AAbbCC;"))
    }

    #[test]
    fn color_add_each_component() {
        assert_eq!("#abbccd", do_evaluate(&[], b"#AbC + 1;"))
    }
    #[test]
    fn color_add_each_component_overflow() {
        assert_eq!("#0101ff", do_evaluate(&[], b"#00f + 1;"))
    }

    #[test]
    fn color_add_components() {
        assert_eq!("#aabbdd", do_evaluate(&[], b"#AbC + #001;"))
    }

    #[test]
    fn color_add_components_overflow() {
        assert_eq!("#1000ff", do_evaluate(&[], b"#1000ff + #001;"))
    }

    #[test]
    fn color_add_components_to_named_overflow() {
        assert_eq!("blue", do_evaluate(&[], b"#0000ff + #001;"))
    }
    #[test]
    fn color_add_components_to_named() {
        assert_eq!("white", do_evaluate(&[], b"#00f + #0f0 + #f00;"))
    }

    #[test]
    fn color_simple_rgba() {
        assert_eq!("rgba(1, 2, 3, 0.6)", do_evaluate(&[], b"rgba(1,2,3,.6);"))
    }

    #[test]
    fn color_add_to_rgba() {
        assert_eq!("#111111", do_evaluate(&[], b"rgba(0, 0, 0, 1) + #111;"))
    }

    #[test]
    fn color_subtract() {
        assert_eq!("#fefefe", do_evaluate(&[], b"#fff - 1;"))
    }

    #[test]
    fn color_subtract_underflow() {
        assert_eq!("black", do_evaluate(&[], b"#000 - 1;"))
    }

    #[test]
    fn color_subtract_components() {
        assert_eq!("#000077", // Or should it be #007?
                   do_evaluate(&[], b"#fff - #ff8;"))
    }

    #[test]
    fn color_subtract_components_underflow() {
        assert_eq!("black", do_evaluate(&[], b"#000001 - #001;"))
    }

    #[test]
    fn color_division() {
        assert_eq!("#020202", do_evaluate(&[], b"(#101010 / 7);"))
    }

    #[test]
    fn color_add_rgb_1() {
        assert_eq!("#0b0a0b", do_evaluate(&[], b"rgb(10,10,10) + #010001;"))
    }
    #[test]
    fn color_add_rgb_2() {
        assert_eq!("white", do_evaluate(&[], b"#010000 + rgb(255, 255, 255);"))
    }

    #[test]
    fn color_named_args() {
        assert_eq!("#010203",
                   do_evaluate(&[], b"rgb($blue: 3, $red: 1, $green: 2);"))
    }

    #[test]
    fn color_mixed_args() {
        assert_eq!("#010203", do_evaluate(&[], b"rgb(1, $blue: 3, $green: 2);"))
    }

    #[test]
    fn color_mixed_with_alpha_1() {
        assert_eq!("rgba(64, 0, 191, 0.75)",
                   do_evaluate(&[], b"mix(rgba(255, 0, 0, 0.5), #00f);"))
    }

    #[test]
    fn color_mixed_with_alpha_2() {
        assert_eq!("rgba(64, 0, 191, 0.75)",
                   do_evaluate(&[], b"mix(#00f, rgba(255, 0, 0, 0.5));"))
    }

    #[test]
    fn value_multiple_dashes() {
        assert_eq!("foo-bar-baz 17%", do_evaluate(&[], b"foo-bar-baz 17%;"))
    }

    #[test]
    fn color_arithemtic_by_name() {
        assert_eq!("magenta", do_evaluate(&[], b"red + blue;"))
    }

    #[test]
    fn function_if() {
        assert_eq!("foo", do_evaluate(&[], b"if(true, foo, bar);"))
    }
    #[test]
    fn function_if_false() {
        assert_eq!("bar", do_evaluate(&[], b"if(false, foo, bar);"))
    }
    #[test]
    fn function_if_named() {
        assert_eq!("hey", do_evaluate(
            &[],
            b"if($if_true: hey, $if_false: ho, $condition: true);"))
    }
    #[test]
    fn function_if_named_dash() {
        assert_eq!("hey", do_evaluate(
            &[],
            b"if($if-true: hey, $if-false: ho, $condition: true);"))
    }

    #[test]
    fn quote_keywords() {
        assert_eq!("\"foo bar\"", do_evaluate(&[], b"quote(foo bar);"))
    }
    #[test]
    fn quote_expr() {
        assert_eq!("\"foo 17\"",
                   do_evaluate(&[("s", "foo"), ("n", "5")],
                               b"quote($s $n * 3 + 2);"))
    }
    #[test]
    fn quoted_string() {
        assert_eq!("\"foobar\"", do_evaluate(&[], b"\"foobar\";"))
    }
    #[test]
    fn unquote_string() {
        assert_eq!("foo bar", do_evaluate(&[], b"unquote(\"foo bar\");"))
    }
    #[test]
    fn unquote_quote() {
        assert_eq!("foo bar", do_evaluate(&[], b"unquote(quote(foo bar));"))
    }

    #[test]
    fn equal_true() {
        assert_eq!("true", do_evaluate(&[], b"17 == 10 + 7;"))
    }
    #[test]
    fn equal_false() {
        assert_eq!("false", do_evaluate(&[], b"17 == 10 + 8;"))
    }
    #[test]
    fn not_equal_true() {
        assert_eq!("true", do_evaluate(&[], b"17 != 10 + 8;"))
    }
    #[test]
    fn not_equal_false() {
        assert_eq!("false", do_evaluate(&[], b"18 != 10 + 8;"))
    }

    #[test]
    fn simple_boolean() {
        assert_eq!("true", do_evaluate(&[], b"3 >= 2 and 1 < 10;"))
    }

    pub fn do_evaluate(s: &[(&str, &str)], expression: &[u8]) -> String {
        let mut scope = ScopeImpl::new();
        for &(name, ref val) in s {
            let val = format!("{};", val);
            let (end, value) = value_expression(val.as_bytes()).unwrap();
            assert_eq!(Ok(";"), from_utf8(end));
            scope.define(name, &value, true)
        }
        let (end, foo) = value_expression(expression).unwrap();
        assert_eq!(Ok(";"), from_utf8(end));
        format!("{}", scope.evaluate(&foo))
    }
}
