use super::{is_not, looks_like_call, CallError, FunctionMap, ResolvedArgs};
use crate::css::{is_calc_name, CallArgs, CssString, Value};
use crate::sass::{Call, Function, MixinDecl, Name};
use crate::value::Quotes;
use crate::{Format, Scope, ScopeRef};

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:meta");
    // - - - Mixins - - -
    f.define_mixin(name!(load_css), MixinDecl::LoadCss);

    // - - - Functions - - -
    def!(f, calc_args(calc), |s| {
        s.get_map(name!(calc), |v| match v {
            Value::Call(name, args) if name == "calc" => {
                // TODO: Maybe allow a single numeric argument to be itself?
                Ok(args.to_string().into())
            }
            Value::Call(name, args) => {
                if is_calc_name(&name) {
                    Ok(args.into())
                } else {
                    Ok(Value::Call(name, args))
                }
            }
            Value::Literal(s) if looks_like_call(&s) => {
                let s = s.value();
                let i = s.find('(').unwrap();
                Ok(s[i + 1..s.len() - 1].into())
            }
            v => Err(is_not(&v, "a calculation")),
        })
    });
    def!(f, calc_name(calc), |s| {
        let name = s.get_map(name!(calc), |v| match v {
            Value::Call(name, _) => Ok(name),
            Value::Literal(s) if looks_like_call(&s) => {
                let s = s.value();
                let i = s.find('(').unwrap();
                Ok(s[..i].to_string())
            }
            v => Err(is_not(&v, "a calculation")),
        })?;
        Ok(CssString::new(name, Quotes::Double).into())
    });
    def_va!(f, call(function, args), |s| {
        let (function, name) = s.get_map(name!(function), |v| match v {
            Value::Function(ref name, ref func) => {
                Ok((Some(func.clone()), name.clone()))
            }
            Value::Literal(name) => {
                dep_warn!(
                    "Passing a string to call() is deprecated and \
                     will be illegal"
                );
                Ok((None, name.value().into()))
            }
            ref v => Err(is_not(v, "a function reference")),
        })?;
        let function = function
            .ok_or(())
            .or_else(|_| get_function(s, None, &name))?;
        let args = s.get_map(name!(args), CallArgs::from_value)?;
        if let Some(function) = function {
            function.call(Call {
                args,
                scope: s.call_scope(),
            })
        } else {
            Ok(Value::Call(name, args))
        }
    });
    def!(f, content_exists(), |s| {
        if let Some(content) = s.call_scope().get_content() {
            Ok((!content.is_no_body()).into())
        } else {
            Err(CallError::msg(
                "content-exists() may only be called within a mixin.",
            ))
        }
    });
    def!(f, feature_exists(feature), |s| {
        let feature: String = s.get(name!(feature))?;
        Ok(IMPLEMENTED_FEATURES.iter().any(|s| s == &feature).into())
    });
    def!(f, function_exists(name, module = b"null"), |s| {
        let name: String = s.get(name!(name))?;
        let module = s.get_opt(name!(module))?;
        Ok(get_function(s, module, &name)?.is_some().into())
    });
    def!(
        f,
        get_function(name, css = b"false", module = b"null"),
        |s| {
            let name: CssString = s.get(name!(name))?;
            let module = s.get_opt(name!(module))?;
            if Value::is_true(&s.get(name!(css))?) {
                if module.is_some() {
                    return Err(CallError::msg(
                        "$css and $module may not both be passed at once.",
                    ));
                }
                Ok(Value::Function(name.take_value(), None))
            } else if let Some(f) = get_function(s, module, name.value())? {
                Ok(Value::Function(name.take_value(), Some(f)))
            } else {
                Err(CallError::msg(format!("Function not found: {}", name)))
            }
        }
    );
    def!(f, global_variable_exists(name, module = b"null"), |s| {
        let name: String = s.get(name!(name))?;
        let module = get_module_arg(s, true)?;
        Ok(module.get_global_or_none(&name.into()).is_some().into())
    });
    def!(f, inspect(value), |s| {
        Ok(s.get::<Value>(name!(value))?
            .format(Format::introspect())
            .to_string()
            .into())
    });
    def!(f, keywords(args), |s| {
        let args = s.get_map(name!(args), |v| match v {
            Value::ArgList(args) => Ok(args.named),
            v => Err(is_not(&v, "an argument list")),
        })?;
        Ok(Value::Map(
            args.into_iter()
                .map(|(k, v)| (k.to_string().into(), v))
                .collect(),
        ))
    });
    def!(f, mixin_exists(name, module = b"null"), |s| {
        let name: String = s.get(name!(name))?;
        let module = get_module_arg(s, true)?;
        Ok(module.get_mixin(&name.into()).is_some().into())
    });
    def!(f, module_functions(module), |s| {
        let module = get_module_arg(s, false)?;
        Ok(module.functions_map())
    });
    def!(f, module_variables(module), |s| {
        let module = get_module_arg(s, false)?;
        Ok(module.variables_map())
    });
    def!(f, type_of(value), |s| {
        Ok(s.get::<Value>(name!(value))?.type_name().into())
    });
    def!(f, variable_exists(name), |s| {
        let name: String = s.get(name!(name))?;
        Ok(s.call_scope().get_or_none(&name.into()).is_some().into())
    });
    f
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        (name!(call), name!(call)),
        (name!(content_exists), name!(content_exists)),
        (name!(feature_exists), name!(feature_exists)),
        (name!(function_exists), name!(function_exists)),
        (name!(get_function), name!(get_function)),
        (name!(global_variable_exists), name!(global_variable_exists)),
        (name!(inspect), name!(inspect)),
        (name!(keywords), name!(keywords)),
        (name!(mixin_exists), name!(mixin_exists)),
        (name!(type_of), name!(type_of)),
        (name!(variable_exists), name!(variable_exists)),
    ] {
        global.insert(gname.clone(), m.get_lfunction(lname));
    }
}

static IMPLEMENTED_FEATURES: &[&str] = &[
    // A local variable will shadow a global variable unless
    // `!global` is used.
    "global-variable-shadowing",
    // "extend-selector-pseudoclass" - nothing with `@extend` is implemented
    // Full support for unit arithmetic using units defined in the
    // [Values and Units Level 3][] spec.
    "units-level-3",
    // The Sass `@error` directive is supported.
    "at-error",
    // The "Custom Properties Level 1" spec is supported. This means
    // that custom properties are parsed statically, with only
    // interpolation treated as SassScript.
    "custom-property",
];

fn get_module_arg(
    s: &ResolvedArgs,
    use_the: bool,
) -> Result<ScopeRef, CallError> {
    get_scope(s, s.get_opt(name!(module))?, use_the)
}

// Note: `the` is compensating for an inconsistensy in sass-spec
fn get_scope(
    s: &ResolvedArgs,
    module: Option<CssString>,
    the: bool,
) -> Result<ScopeRef, CallError> {
    if let Some(module) = module {
        s.call_scope().get_module(module.value()).ok_or_else(|| {
            CallError::msg(format!(
                "There is no module with {}namespace {}.",
                if the { "the " } else { "" },
                module
            ))
        })
    } else {
        Ok(s.call_scope())
    }
}

fn get_function(
    s: &ResolvedArgs,
    module: Option<CssString>,
    name: &str,
) -> Result<Option<Function>, CallError> {
    let name = Name::from(name);
    if let Some(module) = module {
        get_scope(s, Some(module), true)?
            .get_function(&name)
            .map_err(CallError::msg)
    } else {
        Ok(s.call_scope()
            .get_function(&name)?
            .or_else(|| Function::get_builtin(&name).cloned()))
    }
}

#[cfg(test)]
mod test {
    use crate::variablescope::test::do_evaluate;

    #[test]
    fn variable_exists_not_null() {
        assert_eq!(
            "true",
            do_evaluate(&[("x", "17")], b"variable-exists(x);")
        )
    }

    #[test]
    fn variable_exists_null() {
        assert_eq!(
            "true",
            do_evaluate(&[("x", "null")], b"variable-exists(x);")
        )
    }

    #[test]
    fn variable_exists_not() {
        assert_eq!("false", do_evaluate(&[], b"variable-exists(x);"))
    }

    #[test]
    fn type_of_number() {
        assert_eq!("number", do_evaluate(&[], b"type_of(100px);"))
    }

    #[test]
    fn type_of_string() {
        assert_eq!("string", do_evaluate(&[], b"type_of(asdf);"))
    }

    #[test]
    fn type_of_color() {
        assert_eq!("color", do_evaluate(&[], b"type_of(#fff);"))
    }

    #[test]
    fn type_of_color_by_name() {
        assert_eq!("color", do_evaluate(&[], b"type_of(red);"))
    }
    #[test]
    fn unitless_a() {
        assert_eq!("true", do_evaluate(&[], b"unitless(100);"))
    }
    #[test]
    fn unitless_b() {
        assert_eq!("false", do_evaluate(&[], b"unitless(100px);"))
    }

    /// From `sass-spec/spec/types-4.0`
    mod types_4_0 {
        use super::do_evaluate;
        #[test]
        fn t01() {
            assert_eq!("color", do_evaluate(&[], b"type_of(red);"))
        }
        #[test]
        fn t02() {
            assert_eq!("string", do_evaluate(&[], b"type_of(\"red\");"))
        }
        #[test]
        fn t03() {
            assert_eq!("color", do_evaluate(&[], b"type_of(#abc);"))
        }
        #[test]
        fn t04() {
            assert_eq!("number", do_evaluate(&[], b"type-of(123);"))
        }
        #[test]
        fn t05() {
            assert_eq!("number", do_evaluate(&[], b"type-of(45px);"))
        }
        #[test]
        fn t06() {
            assert_eq!("number", do_evaluate(&[], b"type-of(98%);"))
        }
        #[test]
        fn t07() {
            assert_eq!("list", do_evaluate(&[], b"type-of(1 2 3);"))
        }
        #[test]
        fn t08() {
            assert_eq!("string", do_evaluate(&[], b"type-of(hey);"))
        }
        #[test]
        fn t09() {
            assert_eq!("string", do_evaluate(&[], b"type-of(\"ho\");"))
        }
        #[test]
        fn t10() {
            assert_eq!("string", do_evaluate(&[], b"type-of(#{1+2}px);"))
        }
        #[test]
        fn t11() {
            assert_eq!("bool", do_evaluate(&[], b"type-of(true);"))
        }
        #[test]
        fn t12() {
            assert_eq!("bool", do_evaluate(&[], b"type-of(false);"))
        }
        #[test]
        fn t13() {
            assert_eq!("number", do_evaluate(&[], b"type-of(45 or false);"))
        }
        #[test]
        fn t14() {
            assert_eq!("string", do_evaluate(&[], b"type-of(#{#abc});"))
        }
        #[test]
        fn t15() {
            assert_eq!("type-of(red)", do_evaluate(&[], b"ty#{pe}-of(red);"))
        }
        #[test]
        fn t17() {
            assert_eq!("aqua", do_evaluate(&[], b"aqua;"))
        }
        #[test]
        fn t18() {
            assert_eq!("aqua", do_evaluate(&[("x", "aqua")], b"$x;"))
        }
        #[test]
        fn t19() {
            assert_eq!("33", do_evaluate(&[], b"#{1+2}+3;"))
        }
        #[test]
        fn t20() {
            assert_eq!("url(number)", do_evaluate(&[], b"url(type-of(3+3));"))
        }
    }
}
