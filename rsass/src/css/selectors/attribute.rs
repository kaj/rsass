use crate::css::CssString;

/// A logical attribute selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Attribute {
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

    pub(super) fn write_to_buf(&self, buf: &mut String) {
        use std::fmt::Write;
        buf.push('[');
        write!(buf, "{}{}{}", self.name, self.op, self.val).unwrap();
        if let Some(m) = self.modifier {
            buf.push(' ');
            buf.push(m);
        }
        buf.push(']');
    }
}

pub(crate) mod parser {
    use super::super::logical::parser::name_opt_ns;
    use super::Attribute;
    use crate::parser::css::css_string_any;
    use crate::parser::{input_to_str, util::opt_spacelike, PResult, Span};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::one_of;
    use nom::combinator::{map, map_res, opt};
    use nom::sequence::{delimited, terminated, tuple};

    pub fn attribute(input: Span) -> PResult<Attribute> {
        delimited(
            terminated(tag("["), opt_spacelike),
            alt((
                map(
                    tuple((
                        terminated(name_opt_ns, opt_spacelike),
                        terminated(
                            map_res(
                                alt((
                                    tag("*="),
                                    tag("|="),
                                    tag("="),
                                    tag("$="),
                                    tag("~="),
                                    tag("^="),
                                )),
                                input_to_str,
                            ),
                            opt_spacelike,
                        ),
                        terminated(css_string_any, opt_spacelike),
                        opt(terminated(
                            one_of(
                                "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz",
                            ),
                            opt_spacelike,
                        )),
                    )),
                    |(name, op, val, modifier)| Attribute {
                        name,
                        op: op.to_owned(),
                        val,
                        modifier,
                    },
                ),
                map(terminated(name_opt_ns, opt_spacelike), |name| {
                    Attribute {
                        name,
                        op: String::new(),
                        val: "".into(),
                        modifier: None,
                    }
                }),
            )),
            tag("]"),
        )(input)
    }
}
