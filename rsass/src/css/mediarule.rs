use super::atrule::AtRuleBodyItem;
use super::Value;
use crate::output::{CssBuf, Format};
use crate::parser::{css::media, input_span};
use crate::value::Operator;
use crate::ParseError;
use std::io::{self, Write};

/// An `@media` rule in css.
#[derive(Clone, Debug)]
pub struct MediaRule {
    args: MediaArgs,
    body: Vec<AtRuleBodyItem>,
}

impl MediaRule {
    pub(crate) fn new(args: MediaArgs, body: Vec<AtRuleBodyItem>) -> Self {
        Self { args, body }
    }
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        if !self.body.is_empty() {
            buf.do_indent_no_nl();
            buf.add_str("@media ");
            self.args.write(buf)?;
            buf.start_block();
            for item in &self.body {
                item.write(buf)?;
            }
            buf.end_block();
        }
        Ok(())
    }
}

/// The media selection argument of an `@media` rule.
#[derive(Clone, Debug)]
pub enum MediaArgs {
    /// A named media, such as `all` or `screen`.
    Name(String),
    /// `(cond: value)` media.
    Condition(String, Box<Value>),
    /// A condition / range, such as `(witdh < 14em)` or
    /// `(14em <= width < 80em)`.
    Range(Vec<(Operator, Value)>),
    /// Any media subquery in parenthesis.
    Paren(Box<MediaArgs>),
    /// Any media subquery in square brackets.
    Bracket(Box<MediaArgs>),
    /// unary operation.
    /// The operator (`not`, `only`) is a string to preserve case.
    UnaryOp(String, Box<MediaArgs>),
    /// a and b and c media.
    Comma(Vec<MediaArgs>),
    /// a and b and c media.
    And(Vec<MediaArgs>),
    /// a or b or c media.
    Or(Vec<MediaArgs>),
}

impl MediaArgs {
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        match self {
            Self::Name(name) => write!(buf, "{name}")?,
            Self::UnaryOp(op, a) => {
                buf.add_str(op);
                buf.add_str(" ");
                a.write(buf)?;
            }
            Self::Condition(c, v) => {
                write!(buf, "({c}: {})", v.format(buf.format()))?;
            }
            Self::Comma(args) => {
                if let Some((first, rest)) = args.split_first() {
                    first.write(buf)?;
                    for arg in rest {
                        buf.add_one(", ", ",");
                        arg.write(buf)?;
                    }
                }
            }
            Self::And(args) => {
                if let Some((first, rest)) = args.split_first() {
                    first.write(buf)?;
                    for arg in rest {
                        buf.add_str(" and ");
                        arg.write(buf)?;
                    }
                }
            }
            Self::Or(args) => {
                if let Some((first, rest)) = args.split_first() {
                    first.write(buf)?;
                    for arg in rest {
                        buf.add_str(" or ");
                        arg.write(buf)?;
                    }
                }
            }
            Self::Paren(a) => {
                buf.add_str("(");
                a.write(buf)?;
                buf.add_str(")");
            }
            Self::Bracket(a) => {
                buf.add_str("[");
                a.write(buf)?;
                buf.add_str("]");
            }
            Self::Range(v) => {
                buf.add_str("(");
                if let Some(((_op, first), rest)) = v.split_first() {
                    buf.add_str(
                        &first.to_string(buf.format()).replace('\n', " "),
                    );
                    for (op, val) in rest {
                        write!(buf, " {op} ")?;
                        buf.add_str(
                            &val.to_string(buf.format()).replace('\n', " "),
                        );
                    }
                }
                buf.add_str(")");
            }
        }
        Ok(())
    }
}

// Note: I'm not sure printing and parsing is the best way to do this,
// but then, I'm not sure media arguments in scss are best represented
// as a Value either.
impl TryFrom<Value> for MediaArgs {
    type Error = crate::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let value = value.format(Format::default()).to_string();
        Ok(ParseError::check(media::args(input_span(value).borrow()))?)
    }
}

/// Check some sizes.
///
/// The exact sizes are not important and may vary between architectures.
/// I just want to keep track that things don't grow too big.
#[cfg(test)]
mod test_sizes {
    use super::*;
    use crate::testutil::test_size;

    test_size!(MediaArgs, 40);
    test_size!(MediaRule, 64);
}
