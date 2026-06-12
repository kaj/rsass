use super::{SassString, Value};
use crate::{Error, ScopeRef, css};

/// A sass definition of a css function.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct CssFunction {
    name: SassString,
    args: Vec<SassString>,
    returns: Option<SassString>,
    body: Vec<FuncItem>,
}

impl CssFunction {
    pub(crate) fn evaluate(
        &self,
        scope: ScopeRef,
    ) -> Result<css::Function, Error> {
        let Self {
            name,
            args,
            returns,
            body,
        } = self;
        Ok(css::Function {
            name: name.evaluate(scope.clone())?.take_value(),
            args: args
                .iter()
                .map(|i| {
                    i.evaluate(scope.clone()).map(css::CssString::take_value)
                })
                .collect::<Result<Vec<_>, _>>()?
                .join(", "),
            returns: returns
                .as_ref()
                .map(|i| i.evaluate(scope.clone()))
                .transpose()?
                .map(css::CssString::take_value),
            body: body
                .iter()
                .map(|i| i.evaluate(scope.clone()))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
enum FuncItem {
    Prop { name: SassString, value: Value },
    Cust { name: SassString, value: SassString },
}

impl FuncItem {
    fn evaluate(&self, scope: ScopeRef) -> Result<css::BodyItem, Error> {
        match self {
            Self::Prop { name, value } => Ok(css::Property::new(
                name.evaluate(scope.clone())?.take_value(),
                value.evaluate(scope)?,
            )
            .into()),
            Self::Cust { name, value } => {
                let name = name.evaluate(scope.clone())?.take_value();
                let value = value.evaluate(scope)?;
                Ok(css::CustomProperty::new(name, value).into())
            }
        }
    }
}

mod parser {
    use crate::parser::strings::{custom_value, sass_string};
    use crate::parser::util::{ignore_comments, semi_or_end};
    use crate::parser::value::value_expression;
    use crate::parser::{PResult, Span};
    use crate::sass::SassString;
    use nom::Parser as _;
    use nom::bytes::complete::tag;
    use nom::character::char;
    use nom::combinator::{map, opt};
    use nom::multi::{many_till, separated_list0};
    use nom::sequence::{delimited, preceded, terminated};

    use super::{CssFunction, FuncItem};

    impl CssFunction {
        pub(crate) fn parse(span: Span, name: SassString) -> PResult<Self> {
            let (end, (args, returns, (body, _))) = (
                delimited(
                    terminated(tag("("), ignore_comments),
                    separated_list0(
                        terminated(tag(","), ignore_comments),
                        map(
                            (
                                terminated(sass_string, ignore_comments),
                                opt(terminated(typedef, ignore_comments)),
                            ),
                            |(mut name, t)| {
                                if let Some(t) = t {
                                    name.append_str(" ");
                                    name.append(&t);
                                }
                                name
                            },
                        ),
                    ),
                    terminated(tag(")"), ignore_comments),
                ),
                opt(delimited(
                    terminated(tag("returns"), ignore_comments),
                    typedef,
                    ignore_comments,
                )),
                preceded(
                    terminated(tag("{"), ignore_comments),
                    many_till(
                        FuncItem::parse,
                        preceded(ignore_comments, tag("}")),
                    ),
                ),
            )
                .parse(span)?;
            Ok((
                end,
                CssFunction {
                    name,
                    args,
                    returns,
                    body,
                },
            ))
        }
    }

    pub fn typedef(input: Span) -> PResult<SassString> {
        map(delimited(char('<'), sass_string, char('>')), |mut t| {
            t.prepend("<");
            t.append_str(">");
            t
        })
        .parse(input)
    }

    impl FuncItem {
        pub fn parse(input: Span) -> PResult<Self> {
            let (rest, name) =
                terminated(sass_string, char(':')).parse(input)?;
            Ok(
                if name.single_raw().is_some_and(|name| {
                    name.eq_ignore_ascii_case("result")
                        || name.starts_with("--")
                }) {
                    let (rest, value) =
                        terminated(custom_value, semi_or_end).parse(rest)?;
                    (rest, Self::Cust { name, value })
                } else {
                    let (rest, value) = delimited(
                        ignore_comments,
                        value_expression,
                        semi_or_end,
                    )
                    .parse(rest)?;
                    (rest, Self::Prop { name, value })
                },
            )
        }
    }
}
