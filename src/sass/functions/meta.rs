use super::{check, get_opt_check, get_string, Error, FunctionMap};
use crate::css::{CallArgs, Value};
use crate::sass::Name;
use crate::{Format, Scope, ScopeRef};

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:meta");
    // - - - Mixins - - -
    // TODO: load_css

    // - - - Functions - - -
    def_va!(f, call(function, args), |s| {
        let (function, name) = match s.get("function")? {
            Value::Function(ref name, ref func) => {
                (func.clone(), name.clone())
            }
            Value::Literal(name, _) => {
                dep_warn!(
                    "Passing a string to call() is deprecated and \
                     will be illegal"
                );
                (call_scope(s).get_function(&(&name).into()), name)
            }
            ref v => {
                return Err(Error::bad_arg(
                    name!(function),
                    v,
                    "is not a function reference",
                ))
            }
        };
        let args = CallArgs::from_value(s.get("args")?);
        if let Some(function) = function {
            function.call(call_scope(s), &args)
        } else {
            Ok(Value::Call(name, args))
        }
    });
    def!(f, content_exists(), |s| {
        let content = s.get_mixin(&Name::from_static("%%BODY%%"));
        Ok(content.map(|m| !m.body.is_empty()).unwrap_or(false).into())
    });
    def!(f, feature_exists(feature), |s| {
        let (feature, _q) = get_string(s, "feature")?;
        Ok(IMPLEMENTED_FEATURES.iter().any(|s| s == &feature).into())
    });
    def!(f, function_exists(name, module = b"null"), |s| {
        let (name, _q) = get_string(s, "name")?;
        let module = get_opt_check(s, name!(module), check::string)?;
        let module = get_scope(s, module.map(|(s, _q)| s), true)?;
        Ok(module.get_function(&name.into()).is_some().into())
    });
    def!(
        f,
        get_function(name, css = b"false", module = b"null"),
        |s| {
            let (v, q) = get_string(s, "name")?;
            let module = get_opt_check(s, name!(module), check::string)?;
            if s.get("css")?.is_true() {
                if module.is_some() {
                    return Err(Error::error(
                        "$css and $module may not both be passed at once",
                    ));
                }
                Ok(Value::Function(v, None))
            } else {
                let module = get_scope(s, module.map(|(s, _q)| s), true)?;
                if let Some(f) = module.get_function(&(&v).into()) {
                    Ok(Value::Function(v, Some(f)))
                } else {
                    Err(Error::S(format!(
                        "Error: Function not found: {}",
                        Value::Literal(v, q).format(Format::introspect())
                    )))
                }
            }
        }
    );
    def!(f, global_variable_exists(name, module = b"null"), |s| {
        let (v, _q) = get_string(s, "name")?;
        let module = get_opt_check(s, name!(module), check::string)?;
        let module = get_scope(s, module.map(|(s, _q)| s), true)?;
        Ok(module.get_global_or_none(&v.into()).is_some().into())
    });
    def!(f, inspect(value), |s| {
        Ok(s.get("value")?
            .format(Format::introspect())
            .to_string()
            .into())
    });
    // TODO: keywords
    def!(f, mixin_exists(name, module = b"null"), |s| {
        let (v, _q) = get_string(s, "name")?;
        let module = get_opt_check(s, name!(module), check::string)?;
        let module = get_scope(s, module.map(|(s, _q)| s), true)?;
        Ok(module.get_mixin(&v.into()).is_some().into())
    });
    def!(f, module_functions(module), |s| {
        let module = get_opt_check(s, name!(module), check::string)?;
        let module = get_scope(s, module.map(|(s, _q)| s), false)?;
        Ok(module.functions_map())
    });
    def!(f, module_variables(module), |s| {
        let module = get_opt_check(s, name!(module), check::string)?;
        let module = get_scope(s, module.map(|(s, _q)| s), false)?;
        Ok(module.variables_map())
    });
    def!(f, type_of(value), |s| {
        Ok(s.get("value")?.type_name().into())
    });
    def!(f, variable_exists(name), |s| {
        let (v, _q) = get_string(s, "name")?;
        Ok(call_scope(s).get_or_none(&v.into()).is_some().into())
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
        // TODO: (name!(keywords), name!(keywords)),
        (name!(mixin_exists), name!(mixin_exists)),
        (name!(type_of), name!(type_of)),
        (name!(variable_exists), name!(variable_exists)),
    ] {
        global.insert(gname.clone(), m.get_function(&lname).unwrap().clone());
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
    // "custom-property",
];

fn call_scope(s: &Scope) -> ScopeRef {
    s.get_module("%%CALLING_SCOPE%%").unwrap()
}

// Note: `the` is compensating for an inconsistensy in sass-spec
fn get_scope(
    s: &Scope,
    module: Option<String>,
    the: bool,
) -> Result<ScopeRef, Error> {
    if let Some(module) = module {
        if let Some(module) = call_scope(s).get_module(&module) {
            Ok(module)
        } else {
            Err(Error::error(format!(
                "There is no module with {}namespace {:?}",
                if the { "the " } else { "" },
                module
            )))
        }
    } else {
        Ok(call_scope(s))
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
