//! A scope is something that contains variable values.

use crate::css::{self, Value};
use crate::functions::{get_builtin_function, Module, SassFunction};
use crate::output::Format;
use crate::sass::{self, Item, Name};
use crate::selectors::Selectors;
use crate::Error;
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
    fn set_variable(
        &mut self,
        name: Name,
        val: Value,
        default: bool,
        global: bool,
    );
    /// Define a none-default, non-global variable.
    fn define(&mut self, name: Name, val: &Value) {
        self.set_variable(name, val.clone(), false, false)
    }
    /// Define a variable in the global scope that is an ultimate
    /// parent of this scope.
    fn define_global(&self, name: Name, val: Value);

    /// Define multiple names from a value that is a list.
    /// Special case: in names is a single name, value is used directly.
    fn define_multi(&mut self, names: &[Name], value: &Value) {
        if names.len() == 1 {
            self.define(names[0].clone(), &value);
        } else {
            let values = value.clone().iter_items();
            if values.len() > names.len() {
                panic!(
                    "Expected {} values, but got {}",
                    names.len(),
                    values.len(),
                )
            } else {
                let mut values = values.iter();
                for name in names {
                    self.define(
                        name.clone(),
                        &values.next().unwrap_or(&Value::Null),
                    )
                }
            }
        }
    }

    /// Get the format used in this scope.
    fn get_format(&self) -> Format;

    /// Get the Value for a variable.
    fn get_or_none(&self, name: &Name) -> Option<Value>;

    /// Get the value for a variable (or an error).
    fn get(&self, name: &str) -> Result<Value, Error> {
        match self.get_or_none(&name.into()) {
            Some(value) => Ok(value),
            None => Err(Error::undefined_variable(name)),
        }
    }

    /// Get the global Value for a variable.
    fn get_global_or_none(&self, name: &Name) -> Option<Value>;

    /// Define a mixin.
    fn define_mixin(
        &mut self,
        name: &str,
        args: &sass::FormalArgs,
        body: &[Item],
    );

    /// Get a mixin by name.
    ///
    /// Returns the formal args and the body of the mixin.
    fn get_mixin(&self, name: &str) -> Option<(sass::FormalArgs, Vec<Item>)>;

    /// Define a function.
    fn define_function(&mut self, name: Name, func: SassFunction);

    /// Get a function by name.
    fn get_function(&self, name: &Name) -> Option<&SassFunction>;
    /// Call a function.
    fn call_function(
        &self,
        name: &Name,
        args: &css::CallArgs,
    ) -> Option<Result<Value, Error>>;

    /// Evaluate a body of items in this scope.
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
                    let range = crate::value::ValueRange::new(
                        from.evaluate(self)?,
                        to.evaluate(self)?,
                        inclusive,
                    )?;
                    for value in range {
                        self.define(name.clone(), &value);
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
                    self.set_variable(name.into(), val, default, global);
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
                ref x => {
                    return Err(Error::S(format!(
                        "Not implemented in fuction: {:?}",
                        x
                    )))
                }
            };
            if let Some(result) = result {
                return Ok(Some(result));
            }
        }
        Ok(None)
    }

    /// Define a module in the scope.
    ///
    /// This is used by the `@use` statement.
    fn define_module(&self, name: Name, module: &'static Module);
    /// Get a module.
    ///
    /// This is used when refering to a function or variable with
    /// namespace.name notation.
    fn get_module(&self, name: &Name) -> Option<&Module>;

    /// Get the selectors active for this scope.
    fn get_selectors(&self) -> &Selectors;
}

pub struct ScopeImpl<'a> {
    parent: &'a dyn Scope,
    variables: BTreeMap<Name, Value>,
    mixins: BTreeMap<String, (sass::FormalArgs, Vec<Item>)>,
    functions: BTreeMap<Name, SassFunction>,
    selectors: Option<Selectors>,
}

impl Scope for ScopeImpl<'_> {
    fn define_module(&self, name: Name, module: &'static Module) {
        self.parent.define_module(name, module);
    }
    fn get_format(&self) -> Format {
        self.parent.get_format()
    }

    fn set_variable(
        &mut self,
        name: Name,
        val: Value,
        default: bool,
        global: bool,
    ) {
        if default
            && !matches!(self.get_or_none(&name), Some(Value::Null) | None)
        {
            return;
        }
        if global {
            self.parent.define_global(name, val);
        } else {
            self.variables.insert(name, val);
        }
    }
    fn define_global(&self, name: Name, val: Value) {
        self.parent.define_global(name, val);
    }
    fn get_mixin(&self, name: &str) -> Option<(sass::FormalArgs, Vec<Item>)> {
        self.mixins
            .get(&name.replace('-', "_"))
            .cloned()
            .or_else(|| self.parent.get_mixin(name))
    }
    fn get_or_none(&self, name: &Name) -> Option<Value> {
        if let Some((modulename, name)) = name.split_module() {
            if let Some(module) = self.get_module(&modulename) {
                return module.get_variable(&name).cloned();
            }
        }
        self.variables
            .get(name)
            .cloned()
            .or_else(|| self.parent.get_or_none(name))
    }
    fn get_global_or_none(&self, name: &Name) -> Option<Value> {
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
    fn define_function(&mut self, name: Name, func: SassFunction) {
        self.functions.insert(name, func);
    }
    fn get_function(&self, name: &Name) -> Option<&SassFunction> {
        if let Some(f) = self.functions.get(name) {
            return Some(f);
        }
        self.parent.get_function(name)
    }
    fn get_module(&self, name: &Name) -> Option<&Module> {
        self.parent.get_module(name)
    }
    fn call_function(
        &self,
        name: &Name,
        args: &css::CallArgs,
    ) -> Option<Result<Value, Error>> {
        if let Some((modulename, name)) = name.split_module() {
            if let Some(module) = self.get_module(&modulename) {
                if let Some(f) = module.get_function(&name).cloned() {
                    return Some(f.call(self, args));
                }
            }
            None
        } else {
            if let Some(f) = self.functions.get(&name).cloned() {
                return Some(f.call(self, args));
            }
            self.parent.call_function(name, args)
        }
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
    variables: Mutex<BTreeMap<Name, Value>>,
    mixins: BTreeMap<String, (sass::FormalArgs, Vec<Item>)>,
    functions: BTreeMap<Name, SassFunction>,
    selectors: Selectors,
    modules: Mutex<BTreeMap<Name, &'static Module>>,
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
            modules: Mutex::new(BTreeMap::new()),
        }
    }
}

impl Scope for GlobalScope {
    fn define_module(&self, name: Name, module: &'static Module) {
        self.modules.lock().unwrap().insert(name, module);
    }
    fn get_module(&self, name: &Name) -> Option<&Module> {
        self.modules.lock().unwrap().get(&name).copied()
    }
    fn get_format(&self) -> Format {
        self.format
    }

    fn set_variable(
        &mut self,
        name: Name,
        val: Value,
        default: bool,
        _global: bool,
    ) {
        if default
            && !matches!(self.get_or_none(&name), Some(Value::Null) | None)
        {
            return;
        }
        self.define_global(name, val);
    }
    fn define_global(&self, name: Name, val: Value) {
        self.variables.lock().unwrap().insert(name, val);
    }

    fn get_mixin(&self, name: &str) -> Option<(sass::FormalArgs, Vec<Item>)> {
        self.mixins.get(&name.replace('-', "_")).cloned()
    }
    fn get_or_none(&self, name: &Name) -> Option<Value> {
        self.get_global_or_none(name)
    }
    fn get_global_or_none(&self, name: &Name) -> Option<Value> {
        self.variables.lock().unwrap().get(name).cloned()
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
    fn define_function(&mut self, name: Name, func: SassFunction) {
        self.functions.insert(name, func);
    }
    fn get_function(&self, name: &Name) -> Option<&SassFunction> {
        if let Some(f) = self.functions.get(name) {
            return Some(f);
        }
        get_builtin_function(name)
    }
    fn call_function(
        &self,
        name: &Name,
        args: &css::CallArgs,
    ) -> Option<Result<Value, Error>> {
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
    macro_rules! assert_expr {
        ($context:expr, $input:expr, $expected:expr) => {{
            assert_eq!(
                do_evaluate_or_error($context, $input)
                    .unwrap_or_else(|e| panic!("{}", e)),
                $expected
            )
        }};
        ($input:expr, $expected:expr) => {{
            assert_expr!(&[], $input, $expected)
        }};
    }

    #[test]
    fn variable_value() {
        assert_expr!(&[("red", "#f02a42")], b"$red;", "#f02a42")
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
        assert_expr!(
            &[("red", "#f02a42")],
            b"solid 1px $red;",
            "solid 1px #f02a42"
        )
    }

    #[test]
    fn simple_arithmetic() {
        assert_expr!(b"3 + 3;", "6")
    }

    #[test]
    fn simple_arithmetic_2() {
        assert_expr!(b"2 + 3 * 4;", "14")
    }

    #[test]
    fn simple_arithmetic_3() {
        assert_expr!(&[("four", "4")], b"2 + 3 * $four;", "14")
    }

    // The following tests are from aboud division are from
    // http://sass-lang.com/documentation/file.SASS_REFERENCE.html ,
    // Section "Divison and /"
    #[test]
    fn div_slash_1() {
        assert_expr!(b"10px/8px;", "10px/8px")
    }

    #[test]
    fn div_slash_2() {
        assert_expr!(&[("width", "1000px")], b"$width/2;", "500px")
    }

    #[test]
    fn div_slash_4() {
        assert_expr!(b"(500px/2);", "250px")
    }

    #[test]
    fn div_slash_5() {
        assert_expr!(b"5px + 8px/2px;", "9px")
    }

    #[test]
    fn div_slash_6() {
        assert_expr!(b"(italic bold 10px/8px);", "italic bold 10px/8px")
    }
    #[test]
    fn negative_in_arithmetic() {
        assert_expr!(&[("m", "20")], b"1000px + $m * -2;", "960px")
    }

    #[test]
    fn double_div_1() {
        assert_expr!(b"15/3/5;", "15/3/5")
    }

    #[test]
    fn double_div_2() {
        assert_expr!(b"15 / 3 / 5;", "15/3/5")
    }

    #[test]
    fn double_div_3() {
        assert_expr!(b"(15 / 3 / 5);", "1")
    }

    #[test]
    fn long_div_and_mul_sequence() {
        assert_expr!(b"(3 / 2 / 2 / 2 * 32 / 2 / 2);", "3")
    }

    #[test]
    fn double_div_4() {
        assert_expr!(b"(15 / 3) / 5;", "1");
    }

    #[test]
    fn double_div_5() {
        assert_expr!(&[("five", "5")], b"15 / 3 / $five;", "1")
    }

    #[test]
    fn sum_w_unit() {
        assert_expr!(b"3px + 3px + 3px;", "9px")
    }
    #[test]
    fn multi_multi() {
        assert_expr!(
            &[("stuff", "1 2 3")],
            b"1 2 3, $stuff 4 5 (6, 7 8 9);",
            "1 2 3, 1 2 3 4 5 6, 7 8 9"
        )
    }

    #[test]
    fn url_keeps_parens() {
        assert_expr!(
            b"black url(starfield.png) repeat;",
            "black url(starfield.png) repeat"
        )
    }

    #[test]
    fn color_unchanged_1() {
        assert_expr!(b"#AbC;", "#AbC")
    }

    #[test]
    fn color_unchanged_2() {
        assert_expr!(b"#AAbbCC;", "#AAbbCC")
    }

    #[test]
    fn color_add_each_component() {
        assert_expr!(b"#AbC + 1;", "#abbccd")
    }
    #[test]
    fn color_add_each_component_overflow() {
        assert_expr!(b"#00f + 1;", "#0101ff")
    }

    #[test]
    fn color_add_components() {
        assert_expr!(b"#AbC + #001;", "#aabbdd")
    }

    #[test]
    fn color_add_components_overflow() {
        assert_expr!(b"#1000ff + #001;", "#1000ff")
    }

    #[test]
    fn color_add_components_to_named_overflow() {
        assert_expr!(b"#0000ff + #001;", "blue")
    }
    #[test]
    fn color_add_components_to_named() {
        assert_expr!(b"#00f + #0f0 + #f00;", "white")
    }

    #[test]
    fn color_simple_rgba() {
        assert_expr!(b"rgba(1,2,3,.6);", "rgba(1, 2, 3, 0.6)")
    }

    #[test]
    fn color_add_to_rgba() {
        assert_expr!(b"rgba(0, 0, 0, 1) + #111;", "#111111")
    }

    #[test]
    fn color_subtract() {
        assert_expr!(b"#fff - 1;", "#fefefe")
    }

    #[test]
    fn color_subtract_underflow() {
        assert_expr!(b"#000 - 1;", "black")
    }

    #[test]
    fn color_subtract_components() {
        assert_expr!(b"#fff - #ff8;", "#000077") // Or should it be #007?
    }

    #[test]
    fn color_subtract_components_underflow() {
        assert_expr!(b"#000001 - #001;", "black")
    }

    #[test]
    fn color_division() {
        assert_expr!(b"(#101010 / 7);", "#020202")
    }

    #[test]
    fn color_add_rgb_1() {
        assert_expr!(b"rgb(10,10,10) + #010001;", "#0b0a0b")
    }
    #[test]
    fn color_add_rgb_2() {
        assert_expr!(b"#010000 + rgb(255, 255, 255);", "white")
    }

    #[test]
    fn color_named_args() {
        assert_expr!(b"rgb($blue: 3, $red: 1, $green: 2);", "#010203")
    }

    #[test]
    fn color_mixed_args() {
        assert_expr!(b"rgb(1, $blue: 3, $green: 2);", "#010203")
    }

    #[test]
    fn color_mixed_with_alpha_1() {
        assert_expr!(
            b"mix(rgba(255, 0, 0, 0.5), #00f);",
            "rgba(64, 0, 191, 0.75)"
        )
    }

    #[test]
    fn color_mixed_with_alpha_2() {
        assert_expr!(
            b"mix(#00f, rgba(255, 0, 0, 0.5));",
            "rgba(64, 0, 191, 0.75)"
        )
    }

    #[test]
    fn value_multiple_dashes() {
        assert_expr!(b"foo-bar-baz 17%;", "foo-bar-baz 17%")
    }

    #[test]
    fn color_arithemtic_by_name() {
        assert_expr!(b"red + blue;", "fuchsia")
    }

    #[test]
    fn function_if() {
        assert_expr!(b"if(true, foo, bar);", "foo")
    }
    #[test]
    fn function_if_false() {
        assert_expr!(b"if(false, foo, bar);", "bar")
    }
    #[test]
    fn function_if_named() {
        assert_expr!(
            b"if($if_true: hey, $if_false: ho, $condition: true);",
            "hey"
        )
    }
    #[test]
    fn function_if_named_dash() {
        assert_expr!(
            b"if($if-true: hey, $if-false: ho, $condition: true);",
            "hey"
        )
    }

    #[test]
    fn quote_keywords() {
        assert_expr!(b"quote(foo bar);", "\"foo bar\"")
    }
    #[test]
    fn quote_expr() {
        let vars = [("s", "foo"), ("n", "5")];
        assert_expr!(&vars, b"quote($s $n * 3 + 2);", "\"foo 17\"")
    }
    #[test]
    fn quoted_string() {
        assert_expr!(b"\"foobar\";", "\"foobar\"")
    }
    #[test]
    fn unquote_string() {
        assert_expr!(b"unquote(\"foo bar\");", "foo bar")
    }
    #[test]
    fn unquote_quote() {
        assert_expr!(b"unquote(quote(foo bar));", "foo bar")
    }

    #[test]
    fn equal_true() {
        assert_expr!(b"17 == 10 + 7;", "true")
    }
    #[test]
    fn equal_false() {
        assert_expr!(b"17 == 10 + 8;", "false")
    }
    #[test]
    fn not_equal_true() {
        assert_expr!(b"17 != 10 + 8;", "true")
    }
    #[test]
    fn not_equal_false() {
        assert_expr!(b"18 != 10 + 8;", "false")
    }

    #[test]
    fn simple_boolean() {
        assert_expr!(b"3 >= 2 and 1 < 10;", "true")
    }

    pub fn do_evaluate(
        s: &[(&'static str, &str)],
        expression: &[u8],
    ) -> String {
        do_evaluate_or_error(s, expression).unwrap()
    }

    pub fn do_evaluate_or_error(
        s: &[(&'static str, &str)],
        expression: &[u8],
    ) -> Result<String, crate::Error> {
        use super::{GlobalScope, Scope};
        use crate::parser::value::value_expression;
        use crate::parser::{code_span, ParseError};
        use crate::sass::Name;
        use nom::bytes::complete::tag;
        use nom::sequence::terminated;
        let mut scope = GlobalScope::new(Default::default());
        for &(name, val) in s {
            let val = value_expression(code_span(val.as_bytes()));
            scope.define(
                Name::from_static(name),
                &ParseError::check(val)?.evaluate(&scope)?,
            );
        }
        let expr =
            terminated(value_expression, tag(";"))(code_span(expression));
        Ok(ParseError::check(expr)?
            .evaluate(&mut scope)?
            .format(scope.get_format())
            .to_string())
    }
}
