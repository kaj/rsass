use crate::{css::CssString, output::CssBuf};

/// A logical attribute selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Attribute {
    /// The attribute name
    name: String,
    /// An operator
    op: String,
    /// A value to match.
    val: CssString,
    /// Optional modifier.
    modifier: Option<char>,
}

impl Attribute {
    pub(super) fn is_superselector(&self, b: &Self) -> bool {
        self.name == b.name
            && self.op == b.op
            && self.val == b.val
            && self.modifier == b.modifier
    }

    pub(super) fn write_to(&self, buf: &mut CssBuf) {
        buf.add_char('[');
        buf.add_str(&self.name);
        buf.add_str(&self.op);
        buf.add_str(&self.val.to_string());
        if let Some(m) = self.modifier {
            buf.add_char(' ');
            buf.add_char(m);
        }
        buf.add_char(']');
    }
}

pub(super) mod parser {
    use super::super::parser::name_opt_ns;
    use super::Attribute;
    use crate::parser::css::strings::css_string_any;
    use crate::parser::util::term_opt_space;
    use crate::parser::{input_to_str, PResult, Span};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::one_of;
    use nom::combinator::{map, map_res, opt};
    use nom::sequence::{delimited, pair, tuple};

    pub(crate) fn attribute(input: Span) -> PResult<Attribute> {
        map(
            delimited(
                term_opt_space(tag("[")),
                pair(
                    term_opt_space(name_opt_ns),
                    opt(tuple((
                        term_opt_space(map_res(
                            alt((
                                tag("*="),
                                tag("|="),
                                tag("="),
                                tag("$="),
                                tag("~="),
                                tag("^="),
                            )),
                            input_to_str,
                        )),
                        term_opt_space(css_string_any),
                        opt(term_opt_space(one_of(
                            "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz",
                        ))),
                    ))),
                ),
                tag("]"),
            ),
            |(name, ovm)| {
                if let Some((op, val, modifier)) = ovm {
                    Attribute {
                        name,
                        op: op.to_owned(),
                        val,
                        modifier,
                    }
                } else {
                    Attribute {
                        name,
                        op: String::new(),
                        val: "".into(),
                        modifier: None,
                    }
                }
            },
        )(input)
    }
}
