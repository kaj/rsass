use css;
use functions::get_builtin_function;
use num_rational::Rational;
use num_traits::{One, Signed, Zero};
use ordermap::OrderMap;
use sass::CallArgs;
use value::{ListSeparator, Operator, Quotes, Unit};
use variablescope::Scope;

/// A sass value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    /// A call has a name and an argument (which may be multi).
    Call(String, CallArgs),
    /// Sometimes an actual division, sometimes "a/b".
    /// In the later case, the booleans tell if there should be whitespace
    /// before / after the slash.
    Div(Box<Value>, Box<Value>, bool, bool),
    Literal(String, Quotes),
    /// A comma- or space separated list of values, with or without brackets.
    List(Vec<Value>, ListSeparator, bool),
    /// A Numeric value is a rational value with a Unit (which may be
    /// Unit::None) and flags.
    ///
    /// The first flag is true for values with an explicit + sign.
    ///
    /// The second flag is true for calculated values and false for
    /// literal values.
    Numeric(Rational, Unit, bool, bool),
    /// "(a/b) and a/b differs semantically.  Parens means the value
    /// should be evaluated numerically if possible, without parens /
    /// is not allways division.
    Paren(Box<Value>),
    Variable(String),
    /// Both a numerical and original string representation,
    /// since case and length should be preserved (#AbC vs #aabbcc).
    Color(Rational, Rational, Rational, Rational, Option<String>),
    Null,
    True,
    False,
    /// A binary operation, two operands and an operator.
    BinOp(Box<Value>, Operator, Box<Value>),
    UnaryOp(Operator, Box<Value>),
    Map(OrderMap<Value, Value>),
    Interpolation(Box<Value>),
}

impl Value {
    pub fn scalar(v: isize) -> Self {
        Value::Numeric(Rational::from_integer(v), Unit::None, false, false)
    }
    pub fn bool(v: bool) -> Self {
        if v { Value::True } else { Value::False }
    }
    pub fn black() -> Self {
        let z = Rational::zero();
        Value::Color(z, z, z, Rational::one(), Some("black".into()))
    }
    pub fn rgba(r: Rational, g: Rational, b: Rational, a: Rational) -> Self {
        fn cap(n: Rational, ff: &Rational) -> Rational {
            if n > *ff {
                *ff
            } else if n.is_negative() {
                Rational::zero()
            } else {
                n
            }
        }
        let ff = Rational::new(255, 1);
        let one = Rational::one();
        Value::Color(cap(r, &ff), cap(g, &ff), cap(b, &ff), cap(a, &one), None)
    }

    pub fn type_name(&self) -> &'static str {
        match *self {
            Value::Color(..) => "color",
            Value::Literal(..) => "string",
            Value::Numeric(..) => "number",
            Value::List(..) => "list",
            Value::True | Value::False => "bool",
            Value::Null => "null",
            _ => "unknown",
        }
    }

    pub fn is_calculated(&self) -> bool {
        match *self {
            Value::Numeric(_, _, _, calculated) => calculated,
            Value::Color(_, _, _, _, None) => true,
            _ => false,
        }
    }

    /// All values other than `False` and `Null` should be considered true.
    pub fn is_true(&self) -> bool {
        match *self {
            Value::False | Value::Null => false,
            _ => true,
        }
    }

    pub fn is_null(&self) -> bool {
        match *self {
            Value::Null => true,
            Value::List(ref list, _, false) => list.iter().all(|v| v.is_null()),
            _ => false,
        }
    }

    pub fn evaluate(&self, scope: &Scope) -> css::Value {
        self.do_evaluate(scope, false)
    }
    pub fn do_evaluate(&self, scope: &Scope, arithmetic: bool) -> css::Value {
        match *self {
            Value::Literal(ref v, ref q) => {
                css::Value::Literal(v.clone(), q.clone())
            }
            Value::Paren(ref v) => v.do_evaluate(scope, true),
            Value::Color(r, g, b, a, ref s) => {
                css::Value::Color(r, g, b, a, s.clone())
            }
            Value::Variable(ref name) => scope.get(name).into_calculated(),
            Value::List(ref v, ref s, b) => {
                css::Value::List(v.iter()
                                     .map(|v| v.do_evaluate(scope, false))
                                     .collect::<Vec<_>>(),
                                 s.clone(),
                                 b)
            }
            Value::Call(ref name, ref args) => {
                match scope.call_function(name, &args.evaluate(scope, true)) {
                    Some(value) => value,
                    None => {
                        if let Some(function) = get_builtin_function(name) {
                            match function.call(scope,
                                                &args.evaluate(scope, true)) {
                                Ok(v) => v,
                                Err(e) => {
                                    panic!("Error in function {}: {:?}",
                                           name, e)
                                }
                            }
                        } else {
                            css::Value::Call(name.clone(),
                                             args.evaluate(scope, false))
                        }
                    }
                }
            }
            Value::Div(ref a, ref b, ref space1, ref space2) => {
                let (a, b) = {
                    let aa = a.do_evaluate(scope, arithmetic);
                    let b =
                        b.do_evaluate(scope, arithmetic || a.is_calculated());
                    if !arithmetic && b.is_calculated() && !a.is_calculated() {
                        (a.do_evaluate(scope, true), b)
                    } else {
                        (aa, b)
                    }
                };
                if arithmetic || a.is_calculated() || b.is_calculated() {
                    match (&a, &b) {
                        (&css::Value::Color(ref r, ref g, ref b, ref a, _),
                         &css::Value::Numeric(ref n, Unit::None, ..)) => {
                            css::Value::rgba(r / n, g / n, b / n, *a)
                        }
                        (&css::Value::Numeric(ref av, ref au, ..),
                         &css::Value::Numeric(ref bv, ref bu, ..)) => {
                            if bv.is_zero() {
                                css::Value::Div(Box::new(a.clone()),
                                                Box::new(b.clone()),
                                                *space1,
                                                *space2)
                            } else if bu == &Unit::None {
                                css::Value::Numeric(av / bv,
                                                    au.clone(),
                                                    false,
                                                    true)
                            } else if au == bu {
                                css::Value::Numeric(av / bv,
                                                    Unit::None,
                                                    false,
                                                    true)
                            } else {
                                css::Value::Div(Box::new(a.clone()),
                                                Box::new(b.clone()),
                                                false,
                                                false)
                            }
                        }
                        (a, b) => {
                            css::Value::Div(Box::new(a.clone()),
                                            Box::new(b.clone()),
                                            false,
                                            false)
                        }
                    }
                } else {
                    css::Value::Div(Box::new(a), Box::new(b), *space1, *space2)
                }
            }
            Value::Numeric(ref v, ref u, ref sign, ref calc) => {
                css::Value::Numeric(*v, u.clone(), *sign, arithmetic || *calc)
            }
            Value::Map(ref m) => {
                css::Value::Map(m.iter()
                                    .map(|&(ref k, ref v)| {
                                             (k.do_evaluate(scope, true),
                                              v.do_evaluate(scope, true))
                                         })
                                    .collect())
            }
            Value::Null => css::Value::Null,
            Value::True => css::Value::True,
            Value::False => css::Value::False,
            Value::BinOp(ref a, ref op, ref b) => {
                op.eval(a.do_evaluate(scope, true), b.do_evaluate(scope, true))
            }
            Value::UnaryOp(ref op, ref v) => {
                let value = v.do_evaluate(scope, true);
                match (op.clone(), value) {
                    (Operator::Not, css::Value::Numeric(v, ..)) => {
                        css::Value::bool(v.is_zero())
                    }
                    (Operator::Not, css::Value::True) => css::Value::False,
                    (Operator::Not, css::Value::False) => css::Value::True,
                    (Operator::Minus, css::Value::Numeric(v, u, ..)) => {
                        css::Value::Numeric(-v, u, false, true)
                    }
                    (Operator::Plus, css::Value::Numeric(v, u, ..)) => {
                        css::Value::Numeric(v, u, true, true)
                    }
                    (op, v) => css::Value::UnaryOp(op, Box::new(v)),
                }
            }
            Value::Interpolation(ref v) => {
                match v.do_evaluate(scope, true).unquote() {
                    css::Value::Null => css::Value::Null,
                    css::Value::Literal(s, _) => {
                        css::Value::Literal(s, Quotes::None)
                    }
                    v => css::Value::Literal(format!("{}", v), Quotes::None),
                }
            }
        }
    }

    pub fn unquote(self) -> Value {
        match self {
            Value::Literal(s, _) => Value::Literal(s, Quotes::None),
            Value::List(list, s, b) => {
                Value::List(list.into_iter().map(|v| v.unquote()).collect(),
                            s,
                            b)
            }
            v => v,
        }
    }
}
