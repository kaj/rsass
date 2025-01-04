use super::{CallArgs, Value};
use crate::output::Formatted;
use crate::value::{ListSeparator, Numeric, Operator};
use std::fmt::{self, Display, Write};

impl Display for Formatted<'_, Value> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self.value {
            Value::Bang(ref s) => write!(out, "!{s}"),
            Value::Literal(ref s) => s.fmt(out),
            Value::Function(ref n, ref _f) => {
                let name = n
                    .chars()
                    .flat_map(|c| match c {
                        '"' => vec!['\\', '"'],
                        c => vec![c],
                    })
                    .collect::<String>();
                write!(out, "get-function(\"{name}\")")
            }
            Value::Numeric(ref num, _) => num.format(self.format).fmt(out),
            Value::Color(ref col, ref name) => {
                if let Some(ref name) = *name {
                    name.fmt(out)
                } else {
                    col.format(self.format).fmt(out)
                }
            }
            Value::List(ref v, sep, brackets) => {
                let sep = sep.unwrap_or_default();
                let introspect = self.format.is_introspection();
                if brackets {
                    out.write_str("[")?;
                } else if introspect && v.is_empty() {
                    return out.write_str("()");
                }
                let t = v
                    .iter()
                    // TODO: Get rid of this filter entirely?
                    .filter(|v| !v.is_null() || sep.is_slash() || introspect)
                    .map(|v| {
                        let needs_paren = match *v {
                            Value::List(ref v, inner, false) => {
                                ((brackets
                                    && (sep < inner.unwrap_or_default()))
                                    || introspect
                                        && (sep <= inner.unwrap_or_default()))
                                    && !(introspect && v.len() < 2)
                                    && (introspect || v.len() != 1)
                            }
                            _ => false,
                        };
                        if needs_paren {
                            format!("({})", v.format(self.format))
                        } else {
                            format!("{}", v.format(self.format))
                        }
                    })
                    .collect::<Vec<_>>();
                if let Some((first, rest)) = t.split_first() {
                    if rest.is_empty()
                        && introspect
                        && sep > ListSeparator::Space
                    {
                        if brackets {
                            write!(out, "{}{}", first, sep.sep(true))?;
                        } else {
                            write!(out, "({}{})", first, sep.sep(true))?;
                        }
                    } else {
                        write!(out, "{first}")?;
                        let sep = sep.sep(self.format.is_compressed());
                        for i in rest {
                            write!(out, "{sep}{i}")?;
                        }
                    }
                }
                if brackets {
                    out.write_str("]")?;
                }
                Ok(())
            }
            Value::Call(ref name, ref arg) => {
                // TODO: Converting calc(1px) to 1px should probably be done earlier.
                // it is for scss, but not when loading plain css.
                if let Some(arg) = is_calc_result(name, arg) {
                    write!(out, "{}", arg.format(self.format))
                } else {
                    write!(out, "{name}({arg})")
                }
            }
            Value::BinOp(ref op) => op.format(self.format).fmt(out),
            Value::UnaryOp(ref op, ref v) => {
                op.fmt(out)?;
                if *op == Operator::Not {
                    out.write_char(' ')?;
                }
                v.format(self.format).fmt(out)
            }
            Value::True => write!(out, "true"),
            Value::False => write!(out, "false"),
            Value::Null => {
                if self.format.is_introspection() {
                    out.write_str("null")
                } else {
                    Ok(())
                }
            }
            Value::Map(ref map) => {
                out.write_char('(')?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        out.write_str(", ")?;
                    }
                    if matches!(
                        k,
                        Value::List(_, Some(ListSeparator::Comma), _)
                    ) && self.format.is_introspection()
                    {
                        write!(out, "({})", k.format(self.format))?;
                    } else {
                        write!(out, "{}", k.format(self.format))?;
                    }
                    out.write_str(": ")?;
                    if matches!(
                        v,
                        Value::List(_, Some(ListSeparator::Comma), _)
                    ) && self.format.is_introspection()
                    {
                        write!(out, "({})", v.format(self.format))?;
                    } else {
                        write!(out, "{}", v.format(self.format))?;
                    }
                }
                out.write_char(')')
            }
            Value::UnicodeRange(ref s) => write!(out, "{s}"),
            Value::Paren(ref v) => {
                out.write_char('(')?;
                v.format(self.format).fmt(out)?;
                out.write_char(')')
            }
            Value::ArgList(ref args) => {
                // Note: named args not included in output.
                if let Some((first, rest)) = args.positional.split_first() {
                    if self.format.is_introspection() && rest.is_empty() {
                        out.write_char('(')?;
                        first.format(self.format).fmt(out)?;
                        out.write_str(",)")?;
                    } else {
                        first.format(self.format).fmt(out)?;
                        let sep = ListSeparator::Comma
                            .sep(self.format.is_compressed());
                        for item in rest {
                            out.write_str(sep)?;
                            item.format(self.format).fmt(out)?;
                        }
                    }
                } else {
                    out.write_str("()")?;
                }
                Ok(())
            }
        }
    }
}

/// If a css `calc` can be evaluated to a static numerical result, return that value.
fn is_calc_result<'a>(name: &str, arg: &'a CallArgs) -> Option<&'a Numeric> {
    if name == "calc" {
        arg.get_single().ok().and_then(|a| match a {
            Value::Numeric(n, _) if n.unit.valid_in_css() => Some(n),
            _ => None,
        })
    } else {
        None
    }
}
