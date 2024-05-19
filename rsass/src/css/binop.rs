use super::{is_function_name, InvalidCss, Value};
use crate::output::{Format, Formatted};
use crate::value::{CssDimensionSet, Numeric, Operator};
use std::fmt::{self, Display, Write};

/// A binary operation.
#[derive(Clone, Debug, Eq, PartialOrd)]
pub struct BinOp {
    a: Value,
    s1: bool,
    op: Operator,
    s2: bool,
    b: Value,
}

impl BinOp {
    pub(crate) fn new(
        a: Value,
        s1: bool,
        op: Operator,
        s2: bool,
        b: Value,
    ) -> Self {
        Self { a, s1, op, s2, b }
    }

    pub(crate) fn op(&self) -> Operator {
        self.op
    }
    pub(crate) fn a(&self) -> &Value {
        &self.a
    }
    pub(crate) fn b(&self) -> &Value {
        &self.b
    }

    /// Get this value, but marked as calculated.
    ///
    /// Make sure arithmetic operators are evaluated.
    pub fn into_calculated(mut self) -> Value {
        self.a = self.a.into_calculated();
        self.b = self.b.into_calculated();
        match self.op.eval(self.a.clone(), self.b.clone()) {
            Ok(Some(v)) => v,
            _ => self.into(),
        }
    }

    /// Validates that this operation is valid in css, even outside of
    /// a `calc(...)` value.
    pub(crate) fn valid_css(mut self) -> Result<Self, InvalidCss> {
        match self.op {
            Operator::Div => {
                self.a = self.a.valid_css()?;
                self.b = self.b.valid_css()?;
                Ok(self)
            }
            Operator::Plus | Operator::Minus => {
                if self.a.is_calculation() || self.b.is_calculation() {
                    Err(InvalidCss::UndefOp(self.into()))
                } else {
                    fn cmp_dim(x: &Numeric) -> Option<CssDimensionSet> {
                        let u = &x.unit;
                        if u.is_known() && !u.is_percent() {
                            Some(u.css_dimension())
                        } else {
                            None
                        }
                    }
                    if let (Value::Numeric(a, _), Value::Numeric(b, _)) =
                        (&self.a, &self.b)
                    {
                        if let (Some(a_u), Some(b_u)) =
                            (cmp_dim(a), cmp_dim(b))
                        {
                            if a_u != b_u {
                                return Err(InvalidCss::Incompat(
                                    a.clone(),
                                    b.clone(),
                                ));
                            }
                        }
                    }
                    Ok(self)
                }
            }
            _ => Err(InvalidCss::UndefOp(self.into())),
        }
    }

    /// Format this operation.
    pub fn format(&self, format: Format) -> Formatted<Self> {
        Formatted {
            value: self,
            format,
        }
    }

    pub(crate) fn add_as_join(&self) -> bool {
        self.op == Operator::Plus
            && (add_as_join(&self.a) || add_as_join(&self.b))
    }
}

impl PartialEq for BinOp {
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op && self.a == other.a && self.b == other.b
    }
}

impl From<BinOp> for Value {
    fn from(value: BinOp) -> Self {
        Self::BinOp(Box::new(value))
    }
}

impl Display for Formatted<'_, BinOp> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        if self.value.add_as_join() {
            // The plus operator is also a concat operator
            self.value.a.format(self.format).fmt(out)?;
            self.value.b.format(self.format).fmt(out)
        } else {
            use Operator::{Div, Minus, Plus};
            fn is_op(v: &Value) -> Option<Operator> {
                match v {
                    Value::BinOp(op) => Some(op.op),
                    _ => None,
                }
            }
            // TODO: This simplification should probably be done in eval.
            let (op, b) = match (self.value.op, &self.value.b) {
                (Plus, Value::Numeric(v, _)) if v.value.is_negative() => {
                    (Minus, Value::from(-v))
                }
                (Minus, Value::Numeric(v, _)) if v.value.is_negative() => {
                    (Plus, Value::from(-v))
                }
                (Div, Value::Numeric(v, c))
                    if !v.value.is_finite() && !v.is_no_unit() =>
                {
                    (
                        Div,
                        Value::Paren(Box::new(Value::Numeric(v.clone(), *c))),
                    )
                }
                (op, Value::Paren(p)) => {
                    if let Some(op2) = is_op(p.as_ref()) {
                        if op2 > op {
                            (op, *p.clone())
                        } else {
                            (op, self.value.b.clone())
                        }
                    } else {
                        (op, self.value.b.clone())
                    }
                }
                (op, Value::BinOp(op2))
                    if ((op2.op < op)
                        || (op == Minus && op2.op == Minus))
                        && !(op.is_cmp() && op2.op.is_cmp()) =>
                {
                    (op, Value::Paren(Box::new(self.value.b.clone())))
                }
                (op, v) => (op, v.clone()),
            };
            self.value.a.format(self.format).fmt(out)?;
            if self.value.s1 {
                out.write_char(' ')?;
            }
            op.fmt(out)?;
            if self.value.s2 {
                out.write_char(' ')?;
            }
            b.format(self.format).fmt(out)
        }
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
