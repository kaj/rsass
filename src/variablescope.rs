//! A scope is something that contains variable values.

use crate::css::{self, Value};
use crate::error::Error;
use crate::functions::{get_builtin_function, SassFunction};
use crate::output::Format;
use crate::sass::{self, Item};
use crate::selectors::Selectors;
use std::collections::BTreeMap;
use std::sync::Mutex;

/// Variables, functions and mixins are defined in a `Scope`.
///
/// A scope can be a local scope, e.g. in a function, or the global scope.
/// All scopes except the global scope has a parent.
/// The global scope is global to a sass document, multiple different
/// global scopes may exists in the same rust-language process.
pub trait Scope {
    /// Define a variable with a value.
    ///
    /// The `$` sign is not included in `name`.
    fn define(&mut self, name: &str, val: &Value);
    fn define_default(&mut self, name: &str, val: &Value, global: bool);
    /// Define a variable in the global scope that is an ultimate
    /// parent of this scope.
    fn define_global(&self, name: &str, val: &Value);

    /// Define multiple names from a value that is a list.
    /// Special case: in names is a single name, value is used directly.
    fn define_multi(&mut self, names: &[String], value: &Value) {
        if names.len() == 1 {
            self.define(&names[0], &value);
        } else if let Value::List(ref values, ..) = *value {
            if values.len() > names.len() {
                panic!(
                    "Expected {} values, but got {}",
                    names.len(),
                    values.len(),
                )
            } else {
                let mut values = values.iter();
                for name in names {
                    self.define(name, &values.next().unwrap_or(&Value::Null))
                }
            }
        } else {
            panic!(
                "Got multiple bindings {:?}, but non-list value {}",
                names,
                value.format(self.get_format())
            )
        }
    }

    fn get_format(&self) -> Format;

    /// Get the Value for a variable.
    fn get_or_none(&self, name: &str) -> Option<Value>;
    fn get(&self, name: &str) -> Result<Value, Error> {
        match self.get_or_none(name) {
            Some(value) => Ok(value),
            None => Err(Error::undefined_variable(name)),
        }
    }

    fn get_global_or_none(&self, name: &str) -> Option<Value>;
    fn get_global(&self, name: &str) -> Result<Value, Error> {
        match self.get_global_or_none(name) {
            Some(value) => Ok(value),
            None => Err(Error::undefined_variable(name)),
        }
    }

    fn define_mixin(
        &mut self,
        name: &str,
        args: &sass::FormalArgs,
        body: &[Item],
    );
    fn get_mixin(&self, name: &str) -> Option<(sass::FormalArgs, Vec<Item>)>;

    fn define_function(&mut self, name: &str, func: SassFunction);
    fn get_function(&self, name: &str) -> Option<&SassFunction>;
    fn call_function(
        &self,
        name: &str,
        args: &css::CallArgs,
    ) -> Option<Result<Value, Error>>;

    fn eval_body(&mut self, body: &[Item]) -> Result<Option<Value>, Error>
    where
        Self: Sized,
    {
        for b in body {
            let result = match *b {
                Item::IfStatement(ref cond, ref do_if, ref do_else) => {
                    if cond.evaluate(self)?.is_true() {
                        self.eval_body(do_if)?
                    } else {
                        self.eval_body(do_else)?
                    }
                }
                Item::Each(ref names, ref values, ref body) => {
                    for value in values.evaluate(self)?.iter_items() {
                        self.define_multi(names, &value);
                        if let Some(r) = self.eval_body(body)? {
                            return Ok(Some(r));
                        }
                    }
                    None
                }
                Item::For {
                    ref name,
                    ref from,
                    ref to,
                    inclusive,
                    ref body,
                } => {
                    let from = from.evaluate(self)?.integer_value().unwrap();
                    let to = to.evaluate(self)?.integer_value().unwrap();
                    let to = if inclusive { to + 1 } else { to };
                    for value in from..to {
                        self.define(name, &Value::scalar(value));
                        if let Some(r) = self.eval_body(body)? {
                            return Ok(Some(r));
                        }
                    }
                    None
                }
                Item::VariableDeclaration {
                    ref name,
                    ref val,
                    default,
                    global,
                } => {
                    let val = val.evaluate(self)?;
                    if default {
                        self.define_default(name, &val, global);
                    } else if global {
                        self.define_global(name, &val);
                    } else {
                        self.define(name, &val);
                    }
                    None
                }
                Item::Return(ref v) => Some(v.evaluate(self)?),
                Item::While(ref cond, ref body) => {
                    let mut scope = ScopeImpl::sub(self);
                    while cond.evaluate(&scope)?.is_true() {
                        if let Some(r) = scope.eval_body(body)? {
                            return Ok(Some(r));
                        }
                    }
                    None
                }
                Item::Warn(ref value) => {
                    eprintln!(
                        "WARNING: {}",
                        value.evaluate(self)?.format(self.get_format())
                    );
                    None
                }
                Item::Error(ref value) => {
                    return Err(Error::S(format!(
                        "Error: {}",
                        value.evaluate(self)?.format(self.get_format()),
                    )));
                }
                Item::None => None,
                Item::Comment(..) => None,
                ref x => panic!("Not implemented in fuction: {:?}", x),
            };
            if let Some(result) = result {
                return Ok(Some(result));
            }
        }
        Ok(None)
    }
    fn get_selectors(&self) -> &Selectors;
}

pub struct ScopeImpl<'a> {
    parent: &'a dyn Scope,
    variables: BTreeMap<String, Value>,
    mixins: BTreeMap<String, (sass::FormalArgs, Vec<Item>)>,
    functions: BTreeMap<String, SassFunction>,
    selectors: Option<Selectors>,
}

impl<'a> Scope for ScopeImpl<'a> {
    fn get_format(&self) -> Format {
        self.parent.get_format()
    }

    fn define(&mut self, name: &str, val: &Value) {
        self.variables
            .insert(name.replace('-', "_"), val.unrequote());
    }
    fn define_default(&mut self, name: &str, val: &Value, global: bool) {
        match self.get_or_none(name) {
            Some(Value::Null) | None => {
                if global {
                    self.define_global(name, val)
                } else {
                    self.define(name, val)
                }
            }
            _ => {}
        }
    }
    fn define_global(&self, name: &str, val: &Value) {
        self.parent.define_global(name, val);
    }
    fn get_mixin(&self, name: &str) -> Option<(sass::FormalArgs, Vec<Item>)> {
        self.mixins
            .get(&name.replace('-', "_"))
            .cloned()
            .or_else(|| self.parent.get_mixin(name))
    }
    fn get_or_none(&self, name: &str) -> Option<Value> {
        let name = name.replace('-', "_");
        self.variables
            .get(&name)
            .cloned()
            .or_else(|| self.parent.get_or_none(&name))
    }
    fn get_global_or_none(&self, name: &str) -> Option<Value> {
        self.parent.get_global_or_none(name)
    }
    fn define_mixin(
        &mut self,
        name: &str,
        args: &sass::FormalArgs,
        body: &[Item],
    ) {
        let name = name.replace('-', "_");
        self.mixins.insert(name, (args.clone(), body.into()));
    }
    fn define_function(&mut self, name: &str, func: SassFunction) {
        self.functions.insert(name.replace('-', "_"), func);
    }
    fn get_function(&self, name: &str) -> Option<&SassFunction> {
        let name = name.replace('-', "_");
        if let Some(f) = self.functions.get(&name) {
            return Some(f);
        }
        self.parent.get_function(&name)
    }
    fn call_function(
        &self,
        name: &str,
        args: &css::CallArgs,
    ) -> Option<Result<Value, Error>> {
        let name = name.replace('-', "_");
        if let Some(f) = self.functions.get(&name).cloned() {
            return Some(f.call(self, args));
        }
        self.parent.call_function(&name, args)
    }
    fn get_selectors(&self) -> &Selectors {
        self.selectors
            .as_ref()
            .unwrap_or_else(|| self.parent.get_selectors())
    }
}

impl<'a> ScopeImpl<'a> {
    pub fn sub(parent: &'a dyn Scope) -> Self {
        ScopeImpl {
            parent,
            variables: BTreeMap::new(),
            mixins: BTreeMap::new(),
            functions: BTreeMap::new(),
            selectors: None,
        }
    }
    pub fn sub_selectors(
        parent: &'a dyn Scope,
        selectors: Selectors,
    ) -> Self {
        ScopeImpl {
            parent,
            variables: BTreeMap::new(),
            mixins: BTreeMap::new(),
            functions: BTreeMap::new(),
            selectors: Some(selectors),
        }
    }
}

/// A `Scope` that can be created without allready having a scope as a
/// parameter is a `GlobalScope`.
///
/// There can be multiple "global" scopes in the same process, they
/// are global to the handling of a scss document.
pub struct GlobalScope {
    format: Format,
    variables: Mutex<BTreeMap<String, Value>>,
    mixins: BTreeMap<String, (sass::FormalArgs, Vec<Item>)>,
    functions: BTreeMap<String, SassFunction>,
    selectors: Selectors,
}

impl GlobalScope {
    /// Create a new global scope.
    pub fn new(format: Format) -> Self {
        GlobalScope {
            format,
            variables: Mutex::new(BTreeMap::new()),
            mixins: BTreeMap::new(),
            functions: BTreeMap::new(),
            selectors: Selectors::root(),
        }
    }
}

impl Scope for GlobalScope {
    fn get_format(&self) -> Format {
        self.format
    }

    fn define(&mut self, name: &str, val: &Value) {
        self.define_global(name, val)
    }
    fn define_default(&mut self, name: &str, val: &Value, _global: bool) {
        match self.get_or_none(name) {
            Some(Value::Null) | None => self.define(name, val),
            _ => {}
        }
    }
    fn define_global(&self, name: &str, val: &Value) {
        self.variables
            .lock()
            .unwrap()
            .insert(name.replace('-', "_"), val.unrequote());
    }
    fn get_mixin(&self, name: &str) -> Option<(sass::FormalArgs, Vec<Item>)> {
        self.mixins.get(&name.replace('-', "_")).cloned()
    }
    fn get_or_none(&self, name: &str) -> Option<Value> {
        self.get_global_or_none(name)
    }
    fn get_global_or_none(&self, name: &str) -> Option<Value> {
        let name = name.replace('-', "_");
        self.variables.lock().unwrap().get(&name).cloned()
    }
    fn define_mixin(
        &mut self,
        name: &str,
        args: &sass::FormalArgs,
        body: &[Item],
    ) {
        let name = name.replace('-', "_");
        self.mixins.insert(name, (args.clone(), body.into()));
    }
    fn define_function(&mut self, name: &str, func: SassFunction) {
        self.functions.insert(name.replace('-', "_"), func);
    }
    fn get_function(&self, name: &str) -> Option<&SassFunction> {
        let name = name.replace('-', "_");
        if let Some(f) = self.functions.get(&name) {
            return Some(f);
        }
        get_builtin_function(&name)
    }
    fn call_function(
        &self,
        name: &str,
        args: &css::CallArgs,
    ) -> Option<Result<Value, Error>> {
        let name = name.replace('-', "_");
        if let Some(f) = self.functions.get(&name).cloned() {
            return Some(f.call(self, args));
        }
        None
    }
    fn get_selectors(&self) -> &Selectors {
        &self.selectors
    }
}

#[cfg(test)]
pub mod test {
    use crate::parser::value::value_expression;
    use crate::variablescope::*;
    use std::str::from_utf8;

    #[test]
    fn variable_value() {
        assert_eq!("#f02a42", do_evaluate(&[("red", "#f02a42")], b"$red;"))
    }

    #[test]
    fn undefined_variable() {
        assert_eq!(
            "Undefined variable: \"$x\"",
            format!("{}", do_evaluate_or_error(&[], b"$x;").err().unwrap())
        )
    }

    #[test]
    fn partial_variable_value() {
        let scope = [("red", "#f02a42")];
        assert_eq!(
            "solid 1px #f02a42",
            do_evaluate(&scope, b"solid 1px $red;")
        )
    }

    #[test]
    fn simple_arithmetic() {
        assert_eq!("6", do_evaluate(&[], b"3 + 3;"))
    }

    #[test]
    fn simple_arithmetic_2() {
        assert_eq!("14", do_evaluate(&[], b"2 + 3 * 4;"))
    }

    #[test]
    fn simple_arithmetic_3() {
        assert_eq!("14", do_evaluate(&[("four", "4")], b"2 + 3 * $four;"))
    }

    // The following tests are from aboud division are from
    // http://sass-lang.com/documentation/file.SASS_REFERENCE.html ,
    // Section "Divison and /"
    #[test]
    fn div_slash_1() {
        assert_eq!("10px/8px", do_evaluate(&[], b"10px/8px;"))
    }

    #[test]
    fn div_slash_2() {
        assert_eq!("500px", do_evaluate(&[("width", "1000px")], b"$width/2;"))
    }

    #[test]
    fn div_slash_4() {
        assert_eq!("250px", do_evaluate(&[], b"(500px/2);"))
    }

    #[test]
    fn div_slash_5() {
        assert_eq!("9px", do_evaluate(&[], b"5px + 8px/2px;"))
    }

    #[test]
    fn div_slash_6() {
        assert_eq!(
            "italic bold 10px/8px",
            do_evaluate(&[], b"(italic bold 10px/8px);")
        )
    }
    #[test]
    fn negative_in_arithmetic() {
        assert_eq!("960px", do_evaluate(&[("m", "20")], b"1000px + $m * -2;"))
    }

    // ...
    #[test]
    fn div_by_zero() {
        assert_eq!("500px/0", do_evaluate(&[], b"(500px/0);"))
    }

    #[test]
    fn double_div_1() {
        assert_eq!("15/3/5", do_evaluate(&[], b"15/3/5;"))
    }

    #[test]
    fn double_div_2() {
        assert_eq!("15 / 3 / 5", do_evaluate(&[], b"15 / 3 / 5;"))
    }

    #[test]
    fn double_div_3() {
        assert_eq!("1", do_evaluate(&[], b"(15 / 3 / 5);"))
    }

    #[test]
    fn long_div_and_mul_sequence() {
        assert_eq!("3", do_evaluate(&[], b"(3 / 2 / 2 / 2 * 32 / 2 / 2);"))
    }

    #[test]
    fn double_div_4() {
        assert_eq!("1", do_evaluate(&[], b"(15 / 3) / 5;"));
    }

    #[test]
    fn double_div_5() {
        assert_eq!("1", do_evaluate(&[("five", "5")], b"15 / 3 / $five;"))
    }

    #[test]
    fn sum_w_unit() {
        assert_eq!("9px", do_evaluate(&[], b"3px + 3px + 3px;"))
    }
    #[test]
    fn multi_multi() {
        let scope = [("stuff", "1 2 3")];
        assert_eq!(
            "1 2 3, 1 2 3 4 5 6, 7 8 9",
            do_evaluate(&scope, b"1 2 3, $stuff 4 5 (6, 7 8 9);")
        )
    }

    #[test]
    fn url_keeps_parens() {
        assert_eq!(
            "black url(starfield.png) repeat",
            do_evaluate(&[], b"black url(starfield.png) repeat;")
        )
    }

    #[test]
    fn color_unchanged_1() {
        assert_eq!("#AbC", do_evaluate(&[], b"#AbC;"))
    }

    #[test]
    fn color_unchanged_2() {
        assert_eq!("#AAbbCC", do_evaluate(&[], b"#AAbbCC;"))
    }

    #[test]
    fn color_add_each_component() {
        assert_eq!("#abbccd", do_evaluate(&[], b"#AbC + 1;"))
    }
    #[test]
    fn color_add_each_component_overflow() {
        assert_eq!("#0101ff", do_evaluate(&[], b"#00f + 1;"))
    }

    #[test]
    fn color_add_components() {
        assert_eq!("#aabbdd", do_evaluate(&[], b"#AbC + #001;"))
    }

    #[test]
    fn color_add_components_overflow() {
        assert_eq!("#1000ff", do_evaluate(&[], b"#1000ff + #001;"))
    }

    #[test]
    fn color_add_components_to_named_overflow() {
        assert_eq!("blue", do_evaluate(&[], b"#0000ff + #001;"))
    }
    #[test]
    fn color_add_components_to_named() {
        assert_eq!("white", do_evaluate(&[], b"#00f + #0f0 + #f00;"))
    }

    #[test]
    fn color_simple_rgba() {
        assert_eq!("rgba(1, 2, 3, 0.6)", do_evaluate(&[], b"rgba(1,2,3,.6);"))
    }

    #[test]
    fn color_add_to_rgba() {
        assert_eq!("#111111", do_evaluate(&[], b"rgba(0, 0, 0, 1) + #111;"))
    }

    #[test]
    fn color_subtract() {
        assert_eq!("#fefefe", do_evaluate(&[], b"#fff - 1;"))
    }

    #[test]
    fn color_subtract_underflow() {
        assert_eq!("black", do_evaluate(&[], b"#000 - 1;"))
    }

    #[test]
    fn color_subtract_components() {
        assert_eq!(
            "#000077", // Or should it be #007?
            do_evaluate(&[], b"#fff - #ff8;")
        )
    }

    #[test]
    fn color_subtract_components_underflow() {
        assert_eq!("black", do_evaluate(&[], b"#000001 - #001;"))
    }

    #[test]
    fn color_division() {
        assert_eq!("#020202", do_evaluate(&[], b"(#101010 / 7);"))
    }

    #[test]
    fn color_add_rgb_1() {
        assert_eq!("#0b0a0b", do_evaluate(&[], b"rgb(10,10,10) + #010001;"))
    }
    #[test]
    fn color_add_rgb_2() {
        assert_eq!(
            "white",
            do_evaluate(&[], b"#010000 + rgb(255, 255, 255);")
        )
    }

    #[test]
    fn color_named_args() {
        assert_eq!(
            "#010203",
            do_evaluate(&[], b"rgb($blue: 3, $red: 1, $green: 2);")
        )
    }

    #[test]
    fn color_mixed_args() {
        assert_eq!(
            "#010203",
            do_evaluate(&[], b"rgb(1, $blue: 3, $green: 2);")
        )
    }

    #[test]
    fn color_mixed_with_alpha_1() {
        assert_eq!(
            "rgba(64, 0, 191, 0.75)",
            do_evaluate(&[], b"mix(rgba(255, 0, 0, 0.5), #00f);")
        )
    }

    #[test]
    fn color_mixed_with_alpha_2() {
        assert_eq!(
            "rgba(64, 0, 191, 0.75)",
            do_evaluate(&[], b"mix(#00f, rgba(255, 0, 0, 0.5));")
        )
    }

    #[test]
    fn value_multiple_dashes() {
        assert_eq!("foo-bar-baz 17%", do_evaluate(&[], b"foo-bar-baz 17%;"))
    }

    #[test]
    fn color_arithemtic_by_name() {
        assert_eq!("magenta", do_evaluate(&[], b"red + blue;"))
    }

    #[test]
    fn function_if() {
        assert_eq!("foo", do_evaluate(&[], b"if(true, foo, bar);"))
    }
    #[test]
    fn function_if_false() {
        assert_eq!("bar", do_evaluate(&[], b"if(false, foo, bar);"))
    }
    #[test]
    fn function_if_named() {
        assert_eq!(
            "hey",
            do_evaluate(
                &[],
                b"if($if_true: hey, $if_false: ho, $condition: true);"
            )
        )
    }
    #[test]
    fn function_if_named_dash() {
        assert_eq!(
            "hey",
            do_evaluate(
                &[],
                b"if($if-true: hey, $if-false: ho, $condition: true);"
            )
        )
    }

    #[test]
    fn quote_keywords() {
        assert_eq!("\"foo bar\"", do_evaluate(&[], b"quote(foo bar);"))
    }
    #[test]
    fn quote_expr() {
        let vars = [("s", "foo"), ("n", "5")];
        assert_eq!("\"foo 17\"", do_evaluate(&vars, b"quote($s $n * 3 + 2);"))
    }
    #[test]
    fn quoted_string() {
        assert_eq!("\"foobar\"", do_evaluate(&[], b"\"foobar\";"))
    }
    #[test]
    fn unquote_string() {
        assert_eq!("foo bar", do_evaluate(&[], b"unquote(\"foo bar\");"))
    }
    #[test]
    fn unquote_quote() {
        assert_eq!("foo bar", do_evaluate(&[], b"unquote(quote(foo bar));"))
    }

    #[test]
    fn equal_true() {
        assert_eq!("true", do_evaluate(&[], b"17 == 10 + 7;"))
    }
    #[test]
    fn equal_false() {
        assert_eq!("false", do_evaluate(&[], b"17 == 10 + 8;"))
    }
    #[test]
    fn not_equal_true() {
        assert_eq!("true", do_evaluate(&[], b"17 != 10 + 8;"))
    }
    #[test]
    fn not_equal_false() {
        assert_eq!("false", do_evaluate(&[], b"18 != 10 + 8;"))
    }

    #[test]
    fn simple_boolean() {
        assert_eq!("true", do_evaluate(&[], b"3 >= 2 and 1 < 10;"))
    }

    pub fn do_evaluate(s: &[(&str, &str)], expression: &[u8]) -> String {
        do_evaluate_or_error(s, expression).unwrap()
    }

    pub fn do_evaluate_or_error(
        s: &[(&str, &str)],
        expression: &[u8],
    ) -> Result<String, Error> {
        let mut scope = GlobalScope::new(Default::default());
        for &(name, ref val) in s {
            let (end, value) = value_expression(val.as_bytes())?;
            let value = value.evaluate(&scope)?;
            assert_eq!(Ok(""), from_utf8(&end));
            scope.define(name, &value);
        }
        let (end, foo) = value_expression(expression)?;
        assert_eq!(Ok(";"), from_utf8(&end));
        Ok(foo
            .evaluate(&mut scope)?
            .format(scope.get_format())
            .to_string())
    }
}
