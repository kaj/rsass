use super::{Name, Value};
use crate::input::SourcePos;
use crate::{Error, ScopeRef};

/// The details of a variable declaration.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct VariableDeclaration {
    /// The variable name
    name: Name,
    /// The bound value
    val: Value,
    /// true if this is a `!default` variable.
    default: bool,
    /// true if this is a `!global` variable.
    global: bool,
    /// The source location of this variable declaration.
    pos: SourcePos,
}

impl VariableDeclaration {
    /// Execute this variable declaration into a scope.
    pub fn evaluate(&self, scope: &ScopeRef) -> Result<(), Error> {
        let val = self.val.evaluate(scope.clone())?;
        scope
            .set_variable(self.name.clone(), val, self.default, self.global)
            .map_err(|e| e.at(self.pos.clone()))
    }
}

pub(crate) mod parser {
    use super::VariableDeclaration;
    #[cfg(test)]
    use crate::parser::check_parse;
    use crate::parser::util::{opt_spacelike, semi_or_end};
    use crate::parser::{name, value::value_expression, PResult, Span};
    use crate::sass::Name;
    #[cfg(test)]
    use crate::sass::Value;
    #[cfg(test)]
    use crate::value::ListSeparator;
    use nom::sequence::{delimited, pair, preceded, terminated};
    use nom::{
        branch::alt, bytes::complete::tag, combinator::map, multi::fold_many0,
    };

    pub(crate) fn variable_declaration(
        input: Span,
    ) -> PResult<VariableDeclaration> {
        preceded(tag("$"), variable_declaration2)(input)
    }

    pub(crate) fn variable_declaration_mod(
        input: Span,
    ) -> PResult<VariableDeclaration> {
        map(
            pair(terminated(name, tag(".")), variable_declaration),
            |(module, decl)| VariableDeclaration {
                name: format!("{}.{}", module, decl.name).into(),
                pos: decl.pos.opt_back(&format!("{}.", module)),
                ..decl
            },
        )(input)
    }

    pub(crate) fn variable_declaration2(
        input0: Span,
    ) -> PResult<VariableDeclaration> {
        let (input, name) = terminated(
            map(name, Name::from),
            delimited(opt_spacelike, tag(":"), opt_spacelike),
        )(input0)?;
        let (input, val) =
            terminated(value_expression, opt_spacelike)(input)?;
        let (input, (default, global)) = fold_many0(
            terminated(
                alt((
                    map(tag("!default"), |_| (true, false)),
                    map(tag("!global"), |_| (false, true)),
                )),
                opt_spacelike,
            ),
            || (false, false),
            |(default, global), (d, g)| (default || d, global || g),
        )(input)?;
        let (trail, _) = semi_or_end(input)?;
        let pos = input0.up_to(&input).to_owned().opt_back("$");
        Ok((
            trail,
            VariableDeclaration {
                name,
                val,
                default,
                global,
                pos,
            }
        ))
    }

    #[test]
    fn test_variable_declaration_simple() {
        let v = check_parse(variable_declaration, b"$foo: bar;").unwrap();
        assert_eq!(
            (v.name, v.val, v.default, v.global),
            ("foo".into(), Value::Literal("bar".into()), false, false)
        )
    }

    #[test]
    fn test_variable_declaration_global() {
        let v = check_parse(variable_declaration, b"$y: some value !global;")
            .unwrap();
        let items = vec![
            Value::Literal("some".into()),
            Value::Literal("value".into()),
        ];
        let list = Value::List(items, Some(ListSeparator::Space), false);
        assert_eq!(
            (v.name, v.val, v.default, v.global),
            ("y".into(), list, false, true,)
        )
    }

    #[test]
    fn test_variable_declaration_default() {
        let v = check_parse(variable_declaration, b"$y: value !default;")
            .unwrap();
        assert_eq!(
            (v.name, v.val, v.default, v.global),
            ("y".into(), Value::Literal("value".into()), true, false,)
        )
    }
}
