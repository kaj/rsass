use super::{Error, FunctionMap, SassFunction};
use crate::css::{CallArgs, Value};
use crate::sass::Name;
use crate::value::Quotes;
use crate::{Format, Scope};

pub fn create_module() -> Scope {
    let f = Scope::new_global(Default::default());
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
                (s.get_function(&(&name).into()), name)
            }
            ref v => return Err(Error::badarg("string", v)),
        };
        let args = CallArgs::from_value(s.get("args")?);
        if let Some(function) = function {
            function.call(
                s.get_module(&Name::from_static("%%CALLING_SCOPE%%"))
                    .unwrap(),
                &args,
            )
        } else {
            Ok(Value::Call(name, args))
        }
    });
    def!(f, content_exists(), |s| {
        let content = s.get_mixin(&Name::from_static("%%BODY%%"));
        Ok(content.map(|m| !m.1.is_empty()).unwrap_or(false).into())
    });
    def!(f, feature_exists(feature), |s| match &s.get("feature")? {
        &Value::Literal(ref v, _) => {
            Ok(IMPLEMENTED_FEATURES.iter().any(|s| s == v).into())
        }
        v => Err(Error::badarg("string", v)),
    });
    def!(f, function_exists(name), |s| match s.get("name")? {
        Value::Literal(v, _) => {
            Ok(s.get_function(&v.into()).is_some().into())
        }
        v => Err(Error::badarg("string", &v)),
    });
    def!(
        f,
        get_function(name, css = b"false"),
        |s| match s.get("name")? {
            Value::Literal(v, _) => {
                if s.get("css")?.is_true() {
                    Ok(Value::Function(v, None))
                } else if let Some(f) = s.get_function(&(&v).into()) {
                    Ok(Value::Function(v, Some(f)))
                } else {
                    Err(Error::S(format!("Function {} does not exist", v)))
                }
            }
            ref v => Err(Error::badarg("string", v)),
        }
    );
    def!(f, global_variable_exists(name), |s| match s.get("name")? {
        Value::Literal(v, _) => {
            Ok(s.get_global_or_none(&v.into()).is_some().into())
        }
        v => Err(Error::badarg("string", &v)),
    });
    def!(f, inspect(value), |s| Ok(Value::Literal(
        match s.get("value")? {
            Value::Null => "null".to_string(),
            Value::List(ref v, _, false) if v.is_empty() => "()".to_string(),
            v => format!("{}", v.format(Format::introspect())),
        },
        Quotes::None
    )));
    // TODO: keywords
    def!(f, mixin_exists(name), |s| match &s.get("name")? {
        &Value::Literal(ref v, _) => {
            Ok(s.get_mixin(&v.into()).is_some().into())
        }
        v => Err(Error::badarg("string", v)),
    });
    // TODO: module_functions, module_variables
    def!(f, type_of(value), |s| Ok(Value::Literal(
        s.get("value")?.type_name().into(),
        Quotes::None
    )));
    def!(f, variable_exists(name), |s| match s.get("name")? {
        Value::Literal(v, _) => {
            Ok(s.get_or_none(&v.into()).is_some().into())
        }
        v => Err(Error::badarg("string", &v)),
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

#[cfg(test)]
mod test {
    use super::super::super::variablescope::test::do_evaluate;

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
        fn t16() {
            assert_eq!(
                "\"length(a b c d)\"",
                do_evaluate(&[], b"quote(le#{ng}th(a b c d));")
            )
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
