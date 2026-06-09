use crate::output::CssBuf;
use std::io;

/// A css `@function` declaration.
///
/// These functions are preserved in css output, not evaluated by rsass.
#[derive(Clone, Debug)]
pub struct Function {
    name: String,
    args: String,            // TODO: Proper type?
    returns: Option<String>, // TODO: Proper type?
    body: Vec<super::BodyItem>,
}

impl Function {
    /// Write this item to a css output buffer.
    pub(crate) fn write(&self, buf: &mut CssBuf) -> io::Result<()> {
        let Self {
            name,
            args,
            returns,
            body,
        } = self;
        buf.do_indent_no_nl();
        buf.add_str("@function ");
        buf.add_str(name);
        buf.add_char('(');
        buf.add_str(args);
        buf.add_char(')');
        if let Some(ret) = returns {
            buf.add_str(" returns ");
            buf.add_str(ret);
        }
        buf.start_block();
        for item in body {
            item.write(buf)?;
        }
        buf.end_block();
        Ok(())
    }
}

mod parser {
    use super::Function;
    use crate::{
        css::{BodyItem, CustomProperty},
        parser::{
            PResult, Span,
            css::{
                comment, import2, opt_spacelike, property_name,
                strings::custom_value,
            },
        },
    };
    use nom::{
        Parser as _,
        branch::alt,
        bytes::complete::tag,
        character::complete::char,
        combinator::{into, map, opt, recognize},
        multi::{many_till, separated_list0},
        sequence::{delimited, preceded, terminated},
    };

    impl Function {
        pub(crate) fn parse(input: Span) -> PResult<Function> {
            map(
                (
                    delimited(opt_spacelike, property_name, opt_spacelike),
                    delimited(
                        terminated(tag("("), opt_spacelike),
                        recognize(separated_list0(
                            comma,
                            (
                                terminated(property_name, opt_spacelike),
                                opt(terminated(typedef, opt_spacelike)),
                            ),
                        )),
                        terminated(tag(")"), opt_spacelike),
                    ),
                    opt(delimited(
                        terminated(tag("returns"), opt_spacelike),
                        typedef,
                        opt_spacelike,
                    )),
                    preceded(
                        terminated(tag("{"), opt_spacelike),
                        many_till(
                            terminated(body_item, opt_spacelike),
                            tag("}"),
                        ),
                    ),
                ),
                |(name, args, ret, (body, _))| Function {
                    name,
                    args: String::from_utf8_lossy(&args).into(),
                    returns: ret.map(|r| String::from_utf8_lossy(&r).into()),
                    body,
                },
            )
            .parse(input)
        }
    }

    pub fn body_item(input: Span) -> PResult<BodyItem> {
        alt((
            into(comment),
            into(preceded(tag("@import"), import2)),
            property,
        ))
        .parse(input)
    }

    pub fn property(input: Span) -> PResult<BodyItem> {
        let (rest, name) =
            terminated(property_name, tag(":")).parse(input)?;
        let (rest, value) =
            terminated(custom_value, opt(tag(";"))).parse(rest)?;
        Ok((rest, CustomProperty::new(name, value).into()))
    }

    pub fn typedef(input: Span) -> PResult<Span> {
        recognize(delimited(char('<'), property_name, char('>'))).parse(input)
    }

    fn comma(input: Span) -> PResult<Span> {
        delimited(opt_spacelike, tag(","), opt_spacelike).parse(input)
    }
}
