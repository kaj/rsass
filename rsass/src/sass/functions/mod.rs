use super::{Call, Closure, FormalArgs, Name};
use crate::css::{self, is_not, BinOp, CallArgs, CssString, Value};
use crate::input::SourcePos;
use crate::output::Format;
use crate::value::{CssDimensionSet, Numeric, Operator, Quotes};
use crate::{Scope, ScopeRef};
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::{cmp, fmt};

#[macro_use]
mod macros;

mod call_error;
mod color;
mod list;
mod map;
mod math;
mod meta;
mod resolvedargs;
mod selector;
mod string;

pub use call_error::CallError;
pub use resolvedargs::ResolvedArgs;

type BuiltinFn =
    dyn Fn(&ResolvedArgs) -> Result<Value, CallError> + Send + Sync;

/// A function that can be called from a sass value.
///
/// The function can be either "builtin" (implemented in rust) or
/// "user defined" (implemented in scss).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Function {
    body: FuncImpl,
}

#[derive(Clone, PartialEq, Eq, PartialOrd)]
enum FuncImpl {
    Builtin(Builtin),
    /// A user-defined function is really a closure, it has a scope
    /// where it is defined and a body of items.
    UserDefined(Closure),
}

#[derive(Clone)]
struct Builtin {
    args: FormalArgs,
    pos: SourcePos,
    body: Arc<BuiltinFn>,
}

impl Builtin {
    fn eval_value(&self, call: Call) -> Result<Value, CallError> {
        let s = self
            .args
            .eval_call(ScopeRef::new_global(call.scope.get_format()), call)
            .map_err(|e| e.declared_at(&self.pos))?;
        (self.body)(&s)
    }
}

impl PartialOrd for Builtin {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self == other {
            Some(cmp::Ordering::Equal)
        } else {
            None
        }
    }
}

impl cmp::PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        // Each builtin function is only created once, so this
        // should be ok.
        self.args == other.args && self.pos == other.pos && {
            #[allow(clippy::vtable_address_comparisons)]
            Arc::ptr_eq(&self.body, &other.body)
        }
    }
}
impl cmp::Eq for Builtin {}

impl fmt::Debug for FuncImpl {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            FuncImpl::Builtin(_) => write!(out, "(builtin function)"),
            FuncImpl::UserDefined(_) => {
                write!(out, "(user-defined function)")
            }
        }
    }
}

trait Functions {
    fn builtin_fn(
        &mut self,
        name: Name,
        args: FormalArgs,
        body: Arc<BuiltinFn>,
    );
}
impl Functions for Scope {
    fn builtin_fn(
        &mut self,
        name: Name,
        args: FormalArgs,
        body: Arc<BuiltinFn>,
    ) {
        let f = Function::builtin(&self.get_name(), &name, args, body);
        self.define_function(name, f);
    }
}

impl Function {
    /// Get a built-in function by name.
    pub fn get_builtin(name: &Name) -> Option<&'static Function> {
        FUNCTIONS.get(name).or_else(|| {
            // Builtin function names are caseless.
            let name = name.as_ref().to_lowercase();
            FUNCTIONS.get(&name.into())
        })
    }

    /// Create a new `Function` from a rust implementation.
    ///
    /// Note: This does not expose the function in any scope, it just
    /// creates it.
    pub fn builtin(
        module: &str,
        name: &Name,
        args: FormalArgs,
        body: Arc<BuiltinFn>,
    ) -> Self {
        let pos = SourcePos::mock_function(name, &args, module);
        Function {
            body: FuncImpl::Builtin(Builtin { args, pos, body }),
        }
    }

    /// Call the function from a given scope and with a given set of
    /// arguments.
    pub fn call(&self, call: Call) -> Result<Value, CallError> {
        let result = match self.body {
            FuncImpl::Builtin(ref builtin) => builtin.eval_value(call),
            FuncImpl::UserDefined(ref closure) => closure.eval_value(call),
        };
        // The result of a calc function is either a calc function call or an
        // "atomic" number, i.e. one that is _not_ marked as "calculated".
        if Some(self) == FUNCTIONS.get(&name!(calc)) {
            match result {
                Ok(Value::Numeric(v, _)) => Ok(Value::Numeric(v, false)),
                result => result,
            }
        } else {
            result.map(Value::into_calculated)
        }
    }
}

impl From<Closure> for Function {
    fn from(c: Closure) -> Self {
        Function {
            body: FuncImpl::UserDefined(c),
        }
    }
}

lazy_static! {
    static ref MODULES: BTreeMap<&'static str, Scope> = {
        let mut modules = BTreeMap::new();
        modules.insert("sass:color", color::create_module());
        modules.insert("sass:list", list::create_module());
        modules.insert("sass:map", map::create_module());
        modules.insert("sass:math", math::create_module());
        modules.insert("sass:meta", meta::create_module());
        modules.insert("sass:selector", selector::create_module());
        modules.insert("sass:string", string::create_module());
        modules
    };
}

/// Get a global module (e.g. `sass:math`) by name.
pub fn get_global_module(name: &str) -> Option<ScopeRef> {
    MODULES.get(name).map(ScopeRef::Builtin)
}

type FunctionMap = BTreeMap<Name, Function>;
impl Functions for FunctionMap {
    fn builtin_fn(
        &mut self,
        name: Name,
        args: FormalArgs,
        body: Arc<BuiltinFn>,
    ) {
        let f = Function::builtin("", &name, args, body);
        self.insert(name, f);
    }
}

lazy_static! {
    static ref FUNCTIONS: FunctionMap = {
        let mut f = BTreeMap::new();
        def!(f, if(condition, if_true, if_false), |s| {
            if Value::is_true(&s.get(name!(condition))?) {
                Ok(s.get(name!(if_true))?)
            } else {
                Ok(s.get(name!(if_false))?)
            }
        });
        def!(f, calc(expr), |s| {
            fn pre_calc(v: &Value) -> bool {
                match v {
                    Value::Numeric(..) => true,
                    Value::Call(ref name, _) => css::is_calc_name(name),
                    Value::Literal(s) => s.is_css_calc(),
                    Value::Paren(s) => pre_calc(s),
                    _ => false,
                }
            }
            fn do_eval(v: Value) -> Result<Value, CallError> {
                match v {
                    Value::Literal(s) if s.quotes() == Quotes::None => {
                        let s = s.value();
                        if s.eq_ignore_ascii_case("e") {
                            Ok(Value::scalar(std::f64::consts::E))
                        } else if s.eq_ignore_ascii_case("pi") {
                            Ok(Value::scalar(std::f64::consts::PI))
                        } else if s.eq_ignore_ascii_case("infinity") {
                            Ok(Value::scalar(f64::INFINITY))
                        } else if s.eq_ignore_ascii_case("-infinity") {
                            Ok(Value::scalar(-f64::INFINITY))
                        } else if s.eq_ignore_ascii_case("NaN") {
                            Ok(Value::scalar(f64::NAN))
                        } else if let Some(arg) = s
                            .strip_prefix("calc(")
                            .and_then(|s| s.strip_suffix(')'))
                        {
                            Ok(Value::Paren(Box::new(arg.into())))
                        } else {
                            Ok(s.into())
                        }
                    }
                    Value::Call(name, args) => {
                        if name == "calc" {
                            let arg = args.get_single().unwrap();
                            match do_eval(arg.clone())? {
                                Value::Literal(s) if s.is_name() => {
                                    Ok(s.into())
                                }
                                arg => Ok(Value::Paren(Box::new(arg))),
                            }
                        } else {
                            Ok(Value::Call(name, args))
                        }
                    }
                    Value::BinOp(op) => {
                        let a = do_eval(op.a().clone())?;
                        let b = do_eval(op.b().clone())?;
                        let op = op.op();
                        if let Ok(Some(result)) =
                            op.eval(a.clone(), b.clone())
                        {
                            return Ok(result);
                        }
                        Ok(BinOp::new(a, true, op, true, b).into())
                    }
                    num @ Value::Numeric(..) => Ok(num),
                    Value::Paren(v) => match v.as_ref() {
                        l @ Value::Paren(_) => Ok(l.clone()),
                        l @ Value::BinOp(..) => Ok(l.clone()),
                        _ => Ok(Value::Paren(v)),
                    },
                    Value::List(v, sep, bra) => {
                        fn seems_numeric(v: &Value) -> bool {
                            match v {
                                Value::Numeric(..) => true,
                                Value::Call(n, _) if n == "calc" => true,
                                Value::Literal(s) => {
                                    // FIXME: This requires more ops, and
                                    // are still a silly way to do it.
                                    s.quotes() == Quotes::None
                                        && !s.value().starts_with('-')
                                        && !s.value().starts_with('+')
                                        && !s.value().ends_with('-')
                                        && !s.value().ends_with('+')
                                }
                                _ => false,
                            }
                        }
                        if v.windows(2).all(|p| p.iter().all(seems_numeric)) {
                            Err(CallError::msg("Missing math operator."))
                        } else {
                            Ok(Value::List(v, sep, bra))
                        }
                    }
                    v => Err(CallError::msg(format!(
                        "Value {} can't be used in a calculation.",
                        v.format(Format::introspect())
                    ))),
                }
            }
            let v = s.get(name!(expr))?;
            let v = do_eval(v)?;
            if pre_calc(&v) {
                match v {
                    Value::Paren(v) => Ok(*v),
                    v => Ok(v),
                }
            } else {
                let arg = css_fn_arg(v)?;
                Ok(Value::Call("calc".into(), CallArgs::from_single(arg)))
            }
        });
        color::expose(MODULES.get("sass:color").unwrap(), &mut f);
        list::expose(MODULES.get("sass:list").unwrap(), &mut f);
        map::expose(MODULES.get("sass:map").unwrap(), &mut f);
        math::expose(MODULES.get("sass:math").unwrap(), &mut f);
        meta::expose(MODULES.get("sass:meta").unwrap(), &mut f);
        selector::expose(MODULES.get("sass:selector").unwrap(), &mut f);
        string::expose(MODULES.get("sass:string").unwrap(), &mut f);
        f
    };
}

fn css_fn_arg(v: Value) -> Result<Value, CallError> {
    match v {
        Value::Literal(s) if s.quotes() == Quotes::None => Ok(s.into()),
        Value::Call(name, args) => Ok(Value::Call(name, args)),
        Value::BinOp(op) => {
            let a = css_fn_arg(op.a().clone())?;
            let b = css_fn_arg(op.b().clone())?;
            let op = op.op();
            if let (Some(adim), Some(bdim)) = (css_dim(&a), css_dim(&b)) {
                if (op == Operator::Plus || op == Operator::Minus)
                    && adim != bdim
                {
                    return Err(CallError::incompatible_values(&a, &b));
                }
            }
            Ok(BinOp::new(a, true, op, true, b).into())
        }
        Value::Numeric(num, c) => {
            if num.unit.valid_in_css() {
                Ok(Value::Numeric(num, c))
            } else {
                Err(CallError::msg(format!(
                    "Number {} isn't compatible with CSS calculations.",
                    Value::Numeric(num, c).introspect()
                )))
            }
        }
        Value::Paren(v) => match v.as_ref() {
            l @ Value::Paren(_) => Ok(l.clone()),
            l @ Value::BinOp(..) => Ok(l.clone()),
            _ => Ok(Value::Paren(v)),
        },
        list @ Value::List(..) => {
            // FIXME: Check if list seems good, as above?
            Ok(list)
        }
        v => Err(CallError::msg(format!(
            "Value {} can't be used in a calculation.",
            v.format(Format::introspect())
        ))),
    }
}

// Note: None here is for unknown, e.g. the dimension of something that is not a number.
fn css_dim(v: &Value) -> Option<CssDimensionSet> {
    match v {
        // TODO: Handle BinOp recursively (again) (or let in_calc return (Value, CssDimension)?)
        Value::Numeric(num, _) => known_dim(num),
        _ => None,
    }
}
fn known_dim(v: &Numeric) -> Option<CssDimensionSet> {
    let u = &v.unit;
    if u.is_known() && !u.is_percent() {
        Some(u.css_dimension())
    } else {
        None
    }
}

// argument helpers for the actual functions

trait CheckedArg<T> {
    fn named(self, name: Name) -> Result<T, CallError>;
}
impl<T, E: ToString> CheckedArg<T> for Result<T, E> {
    fn named(self, name: Name) -> Result<T, CallError> {
        self.map_err(|e| CallError::BadArgument(name, e.to_string()))
    }
}

fn expected_to<T: Into<Value>>(value: T, cond: &str) -> String {
    format!("Expected {} to {}.", value.into().introspect(), cond)
}

fn is_special(v: &Value) -> bool {
    match v {
        Value::Call(..) => true,
        Value::Literal(s) if looks_like_call(s) => true,
        Value::BinOp(..) => true,
        _ => false,
    }
}

fn looks_like_call(s: &CssString) -> bool {
    s.quotes().is_none()
        && s.value().contains('(')
        && s.value().ends_with(')')
}

mod check {
    use super::{expected_to, is_not};
    use crate::css::Value;
    use crate::value::{ListSeparator, Number, Numeric};

    pub fn int(v: Value) -> Result<i64, String> {
        Numeric::try_from(v)?
            .value
            .into_integer()
            .map_err(|v| is_not(&v, "an int"))
    }

    pub fn unitless(v: Value) -> Result<Number, String> {
        let val = Numeric::try_from(v)?;
        if val.is_no_unit() {
            Ok(val.value)
        } else {
            Err(expected_to(val, "have no units"))
        }
    }
    pub fn unitless_int(v: Value) -> Result<i64, String> {
        unitless(v)?
            .into_integer()
            .map_err(|v| is_not(&v, "an int"))
    }

    pub fn va_list(v: Value) -> Result<Vec<Value>, String> {
        match v {
            Value::ArgList(args) => {
                args.check_no_named().map_err(|e| e.to_string())?;
                Ok(args.positional)
            }
            Value::List(v, Some(ListSeparator::Comma), _) => Ok(v),
            single => Ok(vec![single]),
        }
    }
    pub fn va_list_nonempty(v: Value) -> Result<Vec<Value>, String> {
        let result = va_list(v)?;
        if result.is_empty() {
            // TODO: Parameterize "selector"?  Or rename fn va_selectors?
            Err("At least one selector must be passed.".into())
        } else {
            Ok(result)
        }
    }
}

#[test]
fn test_rgb() -> Result<(), Box<dyn std::error::Error>> {
    use crate::parser::code_span;
    use crate::parser::formalargs::call_args;
    use crate::value::Rgba;
    let scope = ScopeRef::new_global(Default::default());
    assert_eq!(
        FUNCTIONS.get(&name!(rgb)).unwrap().call(
            call_args(code_span(b"(17, 0, 225)").borrow())
                .unwrap()
                .1
                .evaluate(scope)?
        )?,
        Value::Color(Rgba::from_rgb(17, 0, 225).into(), None)
    );
    Ok(())
}

#[test]
fn test_nth() {
    assert_eq!("foo", do_evaluate(&[("x", "foo, bar")], b"nth($x, 1);"))
}

#[cfg(test)]
use crate::variablescope::test::do_evaluate;
