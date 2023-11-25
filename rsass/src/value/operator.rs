use crate::css::{is_function_name, CssString, InvalidCss, Value};
use crate::value::{ListSeparator, Numeric};
use std::fmt;

/// An operator that can be used in a sass value.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    /// The boolean (value propagating) `and` operator.
    And,
    /// The boolean (value propagating) `or` operator.
    Or,
    /// The `==` operator.
    Equal,
    /// The `=` operator.  Used insted of Equal in media queries.
    EqualSingle,
    /// The `!=` operator.
    NotEqual,
    /// The `>` operator.
    Greater,
    /// The `>=` operator.
    GreaterE,
    /// The `<` operator.
    Lesser,
    /// The `<=` operator.
    LesserE,

    /// The `+` operator.  Also unary positive.
    Plus,
    /// The `-` operator.  Also unary negative.
    Minus,
    /// The `*` operator.
    Multiply,
    /// The `/` operator.
    Div,
    /// The `%` operator.
    Modulo,

    /// The unary `not` operator.
    Not,
}

impl Operator {
    /// Evaluate this operator with two operands.
    ///
    /// Some operations are known to be invalid, and for those an
    /// error is returned.
    /// Other operations cannot be evaluated but may be valid as is
    /// e.g. in a `calc(...)` value.  In that case, eval returns
    /// Ok(None).
    pub fn eval(&self, a: Value, b: Value) -> Result<Option<Value>, BadOp> {
        fn cmp(
            a: Value,
            b: Value,
            op: &dyn Fn(Value, Value) -> bool,
        ) -> Option<Value> {
            match (&a, &b) {
                (Value::Numeric(..), Value::Numeric(..))
                | (Value::Literal(_), Value::Literal(..)) => {
                    Some(Value::from(op(a, b)))
                }
                _ => None,
            }
        }
        Ok(match *self {
            Operator::And => Some(if a.is_true() { b } else { a }),
            Operator::Or => Some(if a.is_true() { a } else { b }),
            Operator::Equal => Some(Value::from(a == b)),
            Operator::EqualSingle => cmp(a, b, &|a, b| a == b),
            Operator::NotEqual => Some(Value::from(a != b)),
            Operator::Greater => cmp(a, b, &|a, b| a > b),
            Operator::GreaterE => cmp(a, b, &|a, b| a >= b),
            Operator::Lesser => cmp(a, b, &|a, b| a < b),
            Operator::LesserE => cmp(a, b, &|a, b| a <= b),
            Operator::Plus => match (a, b) {
                (Value::Numeric(a, _), Value::Numeric(b, _)) => {
                    if a.unit == b.unit || b.is_no_unit() {
                        Some(Numeric::new(a.value + b.value, a.unit).into())
                    } else if a.is_no_unit() {
                        Some(Numeric::new(a.value + b.value, b.unit).into())
                    } else if let Some(scaled) = b.as_unitset(&a.unit) {
                        Some(Numeric::new(a.value + scaled, a.unit).into())
                    } else {
                        None
                    }
                }
                (Value::Literal(a), Value::Literal(b)) => {
                    let val = format!("{}{}", a.value(), b.value());
                    Some(CssString::new(val, a.quotes()).into())
                }
                (Value::Literal(a), b) if !a.is_css_fn() => {
                    let join = format!(
                        "{}{}",
                        a.value(),
                        b.format(Default::default())
                    );
                    Some(CssString::new(join, a.quotes()).into())
                }
                (a, Value::Literal(b)) if !b.is_css_fn() => {
                    let join = format!(
                        "{}{}",
                        a.format(Default::default()),
                        b.value()
                    );
                    Some(CssString::new(join, b.quotes()).into())
                }
                (a, b) => {
                    if (add_as_join(&a) || add_as_join(&b))
                        || (valid_operand(&a) && valid_operand(&b))
                    {
                        None
                    } else {
                        a.valid_css()?;
                        b.valid_css()?;
                        return Err(BadOp::UndefinedOperation);
                    }
                }
            },
            Operator::Minus => match (a, b) {
                (Value::Numeric(a, _), Value::Numeric(b, _)) => {
                    if a.unit == b.unit || b.is_no_unit() {
                        Some(Numeric::new(&a.value - &b.value, a.unit).into())
                    } else if a.is_no_unit() {
                        Some(Numeric::new(&a.value - &b.value, b.unit).into())
                    } else if let Some(scaled) = b.as_unitset(&a.unit) {
                        Some(Numeric::new(&a.value - &scaled, a.unit).into())
                    } else {
                        None
                    }
                }
                // Note: This very special case should probably be much
                // more general.
                (a @ Value::UnicodeRange(..), b @ Value::Literal(..)) => {
                    Some(Value::List(
                        vec![a, Value::UnaryOp(Operator::Minus, Box::new(b))],
                        Some(ListSeparator::Space),
                        false,
                    ))
                }
                (Value::UnicodeRange(..), Value::Numeric(..)) => None,
                (a, b) => {
                    if (add_as_join(&a) || add_as_join(&b))
                        || (valid_operand(&a) && valid_operand(&b))
                    {
                        None
                    } else {
                        a.valid_css()?;
                        b.valid_css()?;
                        return Err(BadOp::UndefinedOperation);
                    }
                }
            },
            Operator::Multiply => match (a, b) {
                (Value::Numeric(ref a, _), Value::Numeric(ref b, _)) => {
                    Some((a * b).into())
                }
                (a, b) if valid_operand(&a) && valid_operand(&b) => None,
                _ => return Err(BadOp::UndefinedOperation),
            },
            Operator::Div => match (a, b) {
                (Value::Color(..), Value::Numeric(..)) => {
                    return Err(BadOp::UndefinedOperation)
                }
                (Value::Color(..), Value::Color(..)) => {
                    return Err(BadOp::UndefinedOperation)
                }
                (Value::Numeric(ref a, a_c), Value::Numeric(ref b, b_c))
                    if a_c || b_c =>
                {
                    Some((a / b).into())
                }
                _ => None,
            },
            Operator::Modulo => match (a, b) {
                (Value::Numeric(a, _), Value::Numeric(b, _)) => {
                    if a.unit == b.unit {
                        Some(Value::scalar(&a.value % &b.value))
                    } else if b.is_no_unit() {
                        Some(Numeric::new(&a.value % &b.value, a.unit).into())
                    } else {
                        None
                    }
                }
                (a, b) if valid_operand(&a) && valid_operand(&b) => None,
                _ => return Err(BadOp::UndefinedOperation),
            },
            Operator::Not => return Err(BadOp::UndefinedOperation),
        })
    }
    pub(crate) fn is_cmp(self) -> bool {
        matches!(
            self,
            Operator::Equal
                | Operator::EqualSingle
                | Operator::NotEqual
                | Operator::Greater
                | Operator::GreaterE
                | Operator::Lesser
                | Operator::LesserE
        )
    }
}

fn valid_operand(v: &Value) -> bool {
    match v {
        Value::Numeric(..)
        | Value::Call(..)
        | Value::BinOp(_)
        | Value::Literal(_) => true,
        Value::Paren(v) => valid_operand(v),
        _ => false,
    }
}

fn add_as_join(v: &Value) -> bool {
    match v {
        Value::List(..) | Value::True | Value::False => true,
        Value::Literal(ref s) => !s.is_css_fn(),
        Value::Call(ref name, _) => !is_function_name(name),
        Value::BinOp(op) => op.add_as_join(),
        _ => false,
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_str(match *self {
            Operator::And => "and",
            Operator::Or => "or",
            Operator::Equal => "==",
            Operator::EqualSingle => "=",
            Operator::NotEqual => "!=",
            Operator::Greater => ">",
            Operator::GreaterE => ">=",
            Operator::Lesser => "<",
            Operator::LesserE => "<=",
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiply => "*",
            Operator::Modulo => "%",
            Operator::Div => "/",
            Operator::Not => "not",
        })
    }
}

/// Something went wrong when evaluating an operator.
pub enum BadOp {
    /// An undefined operation (e.g. adding incompatible units) were attempted.
    UndefinedOperation,
    /// A potentially valid operation, but with invalid operands.
    Invalid(InvalidCss),
}

impl From<InvalidCss> for BadOp {
    fn from(value: InvalidCss) -> Self {
        BadOp::Invalid(value)
    }
}
