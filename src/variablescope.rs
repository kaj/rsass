//! A scope is something that contains variable values.
use crate::css::{CssString, Selectors, Value};
use crate::error::Invalid;
use crate::output::Format;
use crate::sass::{Expose, Function, Item, MixinDecl, Name, UseAs};
use crate::{Error, SourcePos};
use arc_swap::ArcSwapOption;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

/// A static or dynamic scope referece.
///
/// This dereferences to a [Scope].
#[derive(Clone)]
pub enum ScopeRef {
    /// The builtin scopes in rsass is static.
    Builtin(&'static Scope),
    /// All other scopes are dynamic.  This uses [Arc] reference counting.
    Dynamic(Arc<Scope>),
}

impl ScopeRef {
    /// Create a new global scope.
    ///
    /// A "global" scope is just a scope that have no parent.
    /// There will be multiple global scopes existing during the
    /// evaluation of a single sass file.
    pub fn new_global(format: Format) -> Self {
        Self::dynamic(Scope::new_global(format))
    }
    /// Create a new subscope of a given parent.
    pub fn sub(parent: ScopeRef) -> Self {
        Self::dynamic(Scope::sub(parent))
    }
    /// Create a new subscope of a given parent with selectors.
    pub fn sub_selectors(parent: ScopeRef, selectors: Selectors) -> Self {
        Self::dynamic(Scope::sub_selectors(parent, selectors))
    }
    fn dynamic(scope: Scope) -> Self {
        ScopeRef::Dynamic(Arc::new(scope))
    }

    /// Check if `a` and `b` references the same scope.
    pub fn is_same(a: &Self, b: &Self) -> bool {
        match (a, b) {
            (ScopeRef::Builtin(a), ScopeRef::Builtin(b)) => {
                std::ptr::eq(a, b)
            }
            (ScopeRef::Dynamic(ref a), ScopeRef::Dynamic(ref b)) => {
                Arc::ptr_eq(a, b)
            }
            _ => false,
        }
    }

    /// Evaluate a body of items in this scope.
    pub fn eval_body(self, body: &[Item]) -> Result<Option<Value>, Error>
    where
        Self: Sized,
    {
        for b in body {
            let result = match *b {
                Item::IfStatement(ref cond, ref do_if, ref do_else) => {
                    if cond.evaluate(self.clone())?.is_true() {
                        self.clone().eval_body(do_if)?
                    } else {
                        self.clone().eval_body(do_else)?
                    }
                }
                Item::Each(ref names, ref values, ref body) => {
                    let s = self.clone();
                    for value in values.evaluate(s.clone())?.iter_items() {
                        s.define_multi(names, value);
                        if let Some(r) = s.clone().eval_body(body)? {
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
                        from.evaluate(self.clone())?,
                        to.evaluate(self.clone())?,
                        inclusive,
                    )?;
                    let s = self.clone();
                    for value in range {
                        s.define(name.clone(), value);
                        if let Some(r) = s.clone().eval_body(body)? {
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
                    let val = val.evaluate(self.clone())?;
                    self.set_variable(name.clone(), val, default, global);
                    None
                }
                Item::Return(ref v, _) => {
                    Some(v.do_evaluate(self.clone(), true)?)
                }
                Item::While(ref cond, ref body) => {
                    let scope = ScopeRef::sub(self.clone());
                    while cond.evaluate(scope.clone())?.is_true() {
                        if let Some(r) = scope.clone().eval_body(body)? {
                            return Ok(Some(r));
                        }
                    }
                    None
                }
                Item::Warn(ref value) => {
                    eprintln!(
                        "WARNING: {}",
                        value
                            .evaluate(self.clone())?
                            .format(self.get_format())
                    );
                    None
                }
                Item::Error(ref value, ref pos) => {
                    return Err(Invalid::AtError(
                        value
                            .evaluate(self)?
                            .format(Format::introspect())
                            .to_string(),
                    )
                    .at(pos.clone()));
                }
                Item::None => None,
                Item::Comment(..) => None,
                ref x => {
                    return Err(Error::S(format!(
                        "Not implemented in function: {:?}",
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

    fn with_forwarded(self) -> Self {
        if let Some(forwarded) = self.opt_forward() {
            let merged = ScopeRef::new_global(self.get_format());
            merged.expose_star(&forwarded);
            merged.expose_star(&self);
            merged
        } else {
            self
        }
    }

    fn expose(self, filter: &Expose) -> Self {
        if filter == &Expose::All {
            self
        } else {
            let result = ScopeRef::new_global(self.get_format());
            for (name, function) in &*self.functions.lock().unwrap() {
                if filter.allow_fun(name) {
                    result.define_function(name.clone(), function.clone());
                }
            }
            for (name, m) in &*self.mixins.lock().unwrap() {
                if filter.allow_fun(name) {
                    result.define_mixin(name.clone(), m.clone());
                }
            }
            for (name, value) in &*self.variables.lock().unwrap() {
                if filter.allow_var(name) {
                    result.define(name.clone(), value.clone());
                }
            }
            result
        }
    }
}

impl Deref for ScopeRef {
    type Target = Scope;
    fn deref(&self) -> &Scope {
        match self {
            ScopeRef::Builtin(m) => m,
            ScopeRef::Dynamic(m) => m,
        }
    }
}

/// Variables, functions and mixins are defined in a `Scope`.
///
/// A scope can be a local scope, e.g. in a function, or the global scope.
/// All non-global scopes have a parent.
/// The global scope is global to a sass document, multiple different
/// global scopes may exists in the same rust-language process.
///
/// Scopes are often accessed through a [`ScopeRef`].
pub struct Scope {
    parent: Option<ScopeRef>,
    modules: Mutex<BTreeMap<String, ScopeRef>>,
    variables: Mutex<BTreeMap<Name, Value>>,
    mixins: Mutex<BTreeMap<Name, MixinDecl>>,
    functions: Mutex<BTreeMap<Name, Function>>,
    selectors: Option<Selectors>,
    forward: Mutex<Option<ScopeRef>>,
    format: Format,
    /// The thing to use for `@content` in a mixin.
    content: ArcSwapOption<MixinDecl>,
    // Set of files currently loading. Only used in the root scope.
    loading: Mutex<BTreeMap<String, SourcePos>>,
}

impl Scope {
    /// Create a new global scope.
    ///
    /// A "global" scope is just a scope that have no parent.
    /// There will be multiple global scopes existing during the
    /// evaluation of a single sass file.
    pub fn new_global(format: Format) -> Self {
        Scope {
            parent: None,
            modules: Mutex::new(BTreeMap::new()),
            variables: Mutex::new(BTreeMap::new()),
            mixins: Mutex::new(BTreeMap::new()),
            functions: Mutex::new(BTreeMap::new()),
            selectors: None,
            forward: Default::default(),
            format,
            content: None.into(),
            loading: Default::default(),
        }
    }
    /// Create a scope for a built-in module.
    pub fn builtin_module(name: &'static str) -> Self {
        let s = Scope::new_global(Default::default());
        s.set_variable(
            Name::from_static("@scope_name@"),
            name.into(),
            false,
            false,
        );
        s
    }
    pub(crate) fn get_name(&self) -> String {
        match self.get_local_or_none(&Name::from_static("@scope_name@")) {
            Some(Value::Literal(s)) => s.value().into(),
            _ => "".into(),
        }
    }
    /// Create a new subscope of a given parent.
    pub fn sub(parent: ScopeRef) -> Self {
        let format = parent.get_format();
        Scope {
            parent: Some(parent),
            modules: Mutex::new(BTreeMap::new()),
            variables: Mutex::new(BTreeMap::new()),
            mixins: Mutex::new(BTreeMap::new()),
            functions: Mutex::new(BTreeMap::new()),
            selectors: None,
            forward: Default::default(),
            format,
            content: None.into(),
            loading: Default::default(),
        }
    }
    /// Create a new subscope of a given parent with selectors.
    pub fn sub_selectors(parent: ScopeRef, selectors: Selectors) -> Self {
        let format = parent.get_format();
        Scope {
            parent: Some(parent),
            modules: Mutex::new(BTreeMap::new()),
            variables: Mutex::new(BTreeMap::new()),
            mixins: Mutex::new(BTreeMap::new()),
            functions: Mutex::new(BTreeMap::new()),
            selectors: Some(selectors),
            forward: Default::default(),
            format,
            content: None.into(),
            loading: Default::default(),
        }
    }

    /// Define a module in the scope.
    ///
    /// This is used by the `@use` statement.
    pub fn define_module(&self, name: String, module: ScopeRef) {
        self.modules.lock().unwrap().insert(name, module);
    }
    /// Get a module.
    ///
    /// This is used when refering to a function or variable with
    /// namespace.name notation.
    pub fn get_module(&self, name: &str) -> Option<ScopeRef> {
        self.modules
            .lock()
            .unwrap()
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_module(name)))
    }
    /// Get the format used in this scope.
    pub fn get_format(&self) -> Format {
        self.format
    }

    /// Define a none-default, non-global variable.
    pub fn define(&self, name: Name, val: Value) {
        self.set_variable(name, val, false, false)
    }

    /// Define a variable with a value.
    ///
    /// The `$` sign is not included in `name`.
    pub fn set_variable(
        &self,
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
            self.define_global(name, val);
        } else {
            self.variables.lock().unwrap().insert(name, val);
        }
    }
    /// Define a variable in the global scope that is an ultimate
    /// parent of this scope.
    pub fn define_global(&self, name: Name, val: Value) {
        if let Some(ref parent) = self.parent {
            parent.define_global(name, val);
        } else {
            self.variables.lock().unwrap().insert(name, val);
        }
    }
    /// Define multiple names from a value that is a list.
    /// Special case: in names is a single name, value is used directly.
    pub(crate) fn define_multi(&self, names: &[Name], value: Value) {
        if names.len() == 1 {
            self.define(names[0].clone(), value);
        } else {
            let values = value.iter_items();
            if values.len() > names.len() {
                panic!(
                    "Expected {} values, but got {}",
                    names.len(),
                    values.len(),
                )
            } else {
                let mut values = values.into_iter();
                for name in names {
                    self.define(
                        name.clone(),
                        values.next().unwrap_or(Value::Null),
                    )
                }
            }
        }
    }

    /// Get the Value for a variable.
    pub fn get_or_none(&self, name: &Name) -> Option<Value> {
        if let Some((modulename, name)) = name.split_module() {
            if let Some(module) = self.get_module(&modulename) {
                return module.get_or_none(&name);
            }
        }
        self.get_local_or_none(name)
    }
    fn get_local_or_none(&self, name: &Name) -> Option<Value> {
        self.variables
            .lock()
            .unwrap()
            .get(name)
            .cloned()
            .or_else(|| {
                self.parent.as_ref().and_then(|p| p.get_local_or_none(name))
            })
    }

    /// Get the value for a variable (or an error).
    pub fn get(&self, name: &Name) -> Result<Value, ScopeError> {
        if let Some((modulename, name)) = name.split_module() {
            self.get_module(&modulename)
                .ok_or_else(|| ScopeError::NoModule(modulename))?
                .get(&name)
        } else {
            self.get_local_or_none(name)
                .ok_or(ScopeError::UndefinedVariable)
        }
    }

    /// Copy a set of local variables to a temporary holder
    pub fn store_local_values(
        &self,
        names: &[Name],
    ) -> Vec<(Name, Option<Value>)> {
        let vars = self.variables.lock().unwrap();
        names
            .iter()
            .map(|name| (name.clone(), vars.get(name).cloned()))
            .collect()
    }
    /// Restore a set of local variables from a temporary holder
    pub fn restore_local_values(&self, data: Vec<(Name, Option<Value>)>) {
        let mut vars = self.variables.lock().unwrap();
        for (name, value) in data {
            if let Some(value) = value {
                vars.insert(name, value);
            } else {
                vars.remove(&name);
            }
        }
    }
    /// Get the global Value for a variable.
    pub fn get_global_or_none(&self, name: &Name) -> Option<Value> {
        if let Some(ref parent) = self.parent {
            parent.get_global_or_none(name)
        } else {
            self.get_or_none(name)
        }
    }

    /// Get a mixin by name.
    ///
    /// Returns the formal args and the body of the mixin.
    pub fn get_mixin(&self, name: &Name) -> Option<MixinDecl> {
        if let Some((modulename, name)) = name.split_module() {
            self.get_module(&modulename)
                .and_then(|m| m.get_mixin(&name))
        } else {
            self.mixins.lock().unwrap().get(name).cloned().or_else(|| {
                self.parent.as_ref().and_then(|p| p.get_mixin(name))
            })
        }
    }
    /// Define a mixin.
    pub fn define_mixin(&self, name: Name, mixin: MixinDecl) {
        self.mixins.lock().unwrap().insert(name, mixin);
    }
    /// Define a function.
    pub fn define_function(&self, name: Name, func: Function) {
        self.functions.lock().unwrap().insert(name, func);
    }
    /// Get a function by name.
    pub fn get_function(
        &self,
        name: &Name,
    ) -> Result<Option<Function>, ScopeError> {
        if let Some((modulename, name)) = name.split_module() {
            if let Some(module) = self.get_module(&modulename) {
                if let Some(f) = module.get_function(&name)? {
                    Ok(Some(f))
                } else {
                    Err(ScopeError::UndefinedFunction)
                }
            } else {
                Err(ScopeError::NoModule(modulename))
            }
        } else {
            let f = self.functions.lock().unwrap().get(name).cloned();
            if let Some(f) = f {
                Ok(Some(f))
            } else if let Some(ref parent) = self.parent {
                parent.get_function(name)
            } else {
                Ok(None)
            }
        }
    }

    /// Only for exposing builtin functions; will panic on unknown.
    pub(crate) fn get_lfunction(&self, name: &Name) -> Function {
        self.functions.lock().unwrap().get(name).unwrap().clone()
    }

    /// Get the selectors active for this scope.
    pub fn get_selectors(&self) -> &Selectors {
        lazy_static! {
            static ref ROOT: Selectors = Selectors::root();
        }
        self.selectors.as_ref().unwrap_or_else(|| {
            self.parent
                .as_ref()
                .map(|p| p.get_selectors())
                .unwrap_or_else(|| &ROOT)
        })
    }

    pub(crate) fn do_use(
        &self,
        module: ScopeRef,
        name: &str,
        as_n: &UseAs,
        expose: &Expose,
    ) -> Result<(), Error> {
        let module = module.with_forwarded();
        match as_n {
            UseAs::KeepName => {
                let name = name
                    .rfind(|c| c == ':' || c == '/')
                    .map(|i| &name[i + 1..])
                    .unwrap_or(name);
                self.define_module(name.into(), module.expose(expose));
            }
            UseAs::Star => {
                self.expose_star(&module.expose(expose));
            }
            UseAs::Name(name) => {
                self.define_module(name.clone(), module.expose(expose));
            }
            UseAs::Prefix(prefix) => {
                for (name, function) in &*module.functions.lock().unwrap() {
                    let name = format!("{}{}", prefix, name).into();
                    if expose.allow_var(&name) {
                        self.define_function(name, function.clone());
                    }
                }
                for (name, value) in &*module.variables.lock().unwrap() {
                    let name = format!("{}{}", prefix, name).into();
                    if expose.allow_fun(&name) {
                        self.define(name, value.clone());
                    }
                }
                for (name, m) in &*module.mixins.lock().unwrap() {
                    let name = format!("{}{}", prefix, name).into();
                    if expose.allow_fun(&name) {
                        self.define_mixin(name, m.clone());
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn expose_star(&self, other: &Scope) {
        for (name, function) in &*other.functions.lock().unwrap() {
            self.define_function(name.clone(), function.clone());
        }
        for (name, value) in &*other.variables.lock().unwrap() {
            self.define(name.clone(), value.clone());
        }
        for (name, m) in &*other.mixins.lock().unwrap() {
            self.define_mixin(name.clone(), m.clone());
        }
    }

    /// Get the functions of this scope as a `Value::Map`.
    pub fn functions_map(&self) -> Value {
        use crate::css::ValueMap;
        use crate::value::Quotes;
        let mut result = ValueMap::new();
        for (name, value) in &*self.functions.lock().unwrap() {
            let name = name.to_string();
            result.insert(
                CssString::new(name.clone(), Quotes::Double).into(),
                Value::Function(name, Some(value.clone())),
            );
        }
        Value::Map(result)
    }
    /// Get the variables of this scope as a `Value::Map`.
    pub fn variables_map(&self) -> Value {
        use crate::css::ValueMap;
        use crate::value::Quotes;
        let mut result = ValueMap::new();
        for (name, value) in &*self.variables.lock().unwrap() {
            if name != &Name::from_static("@scope_name@") {
                result.insert(
                    CssString::new(name.to_string(), Quotes::Double).into(),
                    value.clone(),
                );
            }
        }
        Value::Map(result)
    }
    /// Get the forward scope for this scope.
    ///
    /// Create a new one if necessary.
    pub fn forward(&self) -> ScopeRef {
        self.forward
            .lock()
            .unwrap()
            .get_or_insert_with(|| ScopeRef::new_global(self.get_format()))
            .clone()
    }
    /// Get the forward scope for this scope.
    ///
    /// Create a new one if necessary.
    pub fn opt_forward(&self) -> Option<ScopeRef> {
        self.forward.lock().unwrap().clone()
    }
    pub(crate) fn lock_loading(
        &self,
        name: &str,
        pos: SourcePos,
    ) -> Result<(), Error> {
        if let Some(parent) = &self.parent {
            parent.lock_loading(name, pos)
        } else if let Some(old) = self
            .loading
            .lock()
            .unwrap()
            .insert(name.into(), pos.clone())
        {
            Err(Error::ImportLoop(pos, old))
        } else {
            Ok(())
        }
    }
    pub(crate) fn unlock_loading(&self, name: &str) {
        if let Some(parent) = &self.parent {
            parent.unlock_loading(name)
        } else {
            self.loading.lock().unwrap().remove(name);
        }
    }

    pub(crate) fn define_content(&self, body: MixinDecl) {
        self.content.store(Some(Arc::new(body)));
    }
    pub(crate) fn get_content(&self) -> Option<MixinDecl> {
        self.content
            .load_full()
            .map(|c| (*c).clone())
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_content()))
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
            do_evaluate_or_error(&[], b"$x;").err().unwrap().to_string(),
            "Error: Undefined variable.\
             \n  ,\
             \n1 | $x;\
             \n  | ^^\
             \n  '\
             \n  (rsass) 1:1  root stylesheet",
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
        assert_expr!(b"rgb($blue: 3, $red: 1, $green: 2);", "rgb(1, 2, 3)")
    }

    #[test]
    fn color_mixed_args() {
        assert_expr!(b"rgb(1, $blue: 3, $green: 2);", "rgb(1, 2, 3)")
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
    fn quoted_string() {
        assert_expr!(b"\"foobar\";", "\"foobar\"")
    }
    #[test]
    fn unquote_string() {
        assert_expr!(b"unquote(\"foo bar\");", "foo bar")
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
        match do_evaluate_or_error(s, expression) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }
    }

    pub fn do_evaluate_or_error(
        s: &[(&'static str, &str)],
        expression: &[u8],
    ) -> Result<String, crate::Error> {
        use super::ScopeRef;
        use crate::parser::value::value_expression;
        use crate::parser::{code_span, ParseError};
        use crate::sass::Name;
        use nom::bytes::complete::tag;
        use nom::sequence::terminated;
        let f = Default::default();
        let scope = ScopeRef::new_global(f);
        for &(name, val) in s {
            let val = value_expression(code_span(val.as_bytes()));
            scope.define(
                Name::from_static(name),
                ParseError::check(val)?.evaluate(scope.clone())?,
            );
        }
        let expr =
            terminated(value_expression, tag(";"))(code_span(expression));
        Ok(ParseError::check(expr)?
            .evaluate(scope)?
            .format(f)
            .to_string())
    }
}

/// Tried to get something that does not exist from a scope.
#[derive(Debug)]
pub enum ScopeError {
    /// Tried to access a module that does not exist.
    NoModule(String),
    /// Tried to access an undefined name.
    UndefinedVariable,
    /// Undefined function.
    UndefinedFunction,
}

impl ScopeError {
    /// Make an Error from a ScopeError at a specific position.
    pub fn at(self, pos: SourcePos) -> Error {
        Invalid::InScope(self).at(pos)
    }
}

use std::fmt::{self, Display};
impl Display for ScopeError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScopeError::NoModule(name) => write!(
                out,
                "There is no module with the namespace {:?}.",
                name
            ),
            ScopeError::UndefinedVariable => "Undefined variable.".fmt(out),
            ScopeError::UndefinedFunction => "Undefined function.".fmt(out),
        }
    }
}
impl From<ScopeError> for Error {
    fn from(err: ScopeError) -> Error {
        Error::error(err.to_string())
    }
}
