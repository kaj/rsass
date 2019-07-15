use crate::css::Value;
use crate::error::Error;
use crate::file_context::FileContext;
use crate::parser::parse_scss_file;
use crate::sass::{FormalArgs, Item};
use crate::selectors::{Selector, SelectorPart, Selectors};
use crate::variablescope::{Scope, ScopeImpl};
use std::fmt;
use std::io::Write;
use std::str::FromStr;

/// Selected target format.
/// Only formats that are variants of this type are supported by rsass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputStyle {
    Expanded,
    Compressed,
}

/// Get an output style from its name.
impl FromStr for OutputStyle {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "compressed" => Ok(OutputStyle::Compressed),
            "expanded" => Ok(OutputStyle::Expanded),
            s => Err(format!("Output style {:?} not supported", s)),
        }
    }
}

static FORMAT_NAMES: [&'static str; 2] = ["Compressed", "Expanded"];

impl OutputStyle {
    /// Get the names of the supported output styles.
    pub fn variants() -> &'static [&'static str] {
        &FORMAT_NAMES
    }
    /// Write a slice of sass items in this format.
    /// The `file_context` is needed if there are `@import` statements
    /// in the sass file.
    pub fn write_root(
        &self,
        items: &[Item],
        globals: &mut Scope,
        file_context: &FileContext,
    ) -> Result<Vec<u8>, Error> {
        let mut result = CssWriter::new(*self);
        for item in items {
            self.handle_root_item(item, globals, file_context, &mut result)?;
        }
        result.get_result()
    }
    fn handle_root_item(
        &self,
        item: &Item,
        scope: &mut Scope,
        file_context: &FileContext,
        result: &mut CssWriter,
    ) -> Result<(), Error> {
        match *item {
            Item::Import(ref name) => {
                let name = name.evaluate(scope)?;
                if let Value::Literal(ref x, _) = name.clone().unquote() {
                    if let Some((sub_context, file)) =
                        file_context.find_file(x.as_ref())
                    {
                        for item in parse_scss_file(&file)? {
                            self.handle_root_item(
                                &item,
                                scope,
                                &sub_context,
                                result,
                            )?;
                        }
                    } else {
                        if x.starts_with('/') {
                            write!(
                                result.to_imports(),
                                "@import {};{}",
                                name,
                                if self.is_compressed() { "" } else { "\n" }
                            )?;
                        } else {
                            write!(
                                result.to_imports(),
                                "@import url({});{}",
                                x,
                                if self.is_compressed() { "" } else { "\n" }
                            )?;
                        }
                    }
                } else {
                    write!(
                        result.to_imports(),
                        "@import {};{}",
                        name,
                        if self.is_compressed() { "" } else { "\n" }
                    )?;
                }
            }
            Item::VariableDeclaration {
                ref name,
                ref val,
                ref default,
                ref global,
            } => {
                let val = val.do_evaluate(scope, true)?;
                if *default {
                    scope.define_default(name, &val, *global);
                } else if *global {
                    scope.define_global(name, &val);
                } else {
                    scope.define(name, &val);
                }
            }
            Item::AtRoot {
                ref selectors,
                ref body,
            } => {
                let selectors = eval_selectors(selectors, scope)?
                    .with_backref(scope.get_selectors().one());
                let mut s1 = vec![];
                let mut s2 = vec![];
                self.handle_body(
                    &mut s1,
                    &mut s2,
                    &mut ScopeImpl::sub_selectors(scope, selectors.clone()),
                    body,
                    file_context,
                    0,
                )?;

                if !s1.is_empty() {
                    if self.is_compressed() {
                        write!(result.to_content(), "{:#}{{", selectors)?;
                    } else {
                        write!(result.to_content(), "{} {{", selectors)?;
                    }
                    self.write_items(result.to_content(), &s1, 2)?;
                    write!(result.to_content(), "}}")?;
                    self.do_indent(result.to_content(), 0)?;
                }
                if !s2.is_empty() {
                    result.to_content().write_all(&s2)?;
                }
            }
            Item::AtRule {
                ref name,
                ref args,
                ref body,
            } => {
                result.do_separate()?;
                let args = args.evaluate(scope)?;
                write!(result.to_content(), "@{}", name)?;
                if !args.is_null() {
                    write!(result.to_content(), " {}", args)?;
                }
                if let Some(ref body) = *body {
                    if self.is_compressed() {
                        write!(result.to_content(), "{{")?;
                    } else {
                        write!(result.to_content(), " {{")?;
                    }
                    let mut direct = vec![];
                    let mut sub = vec![];
                    self.handle_body(
                        &mut direct,
                        &mut sub,
                        &mut ScopeImpl::sub(scope),
                        body,
                        file_context,
                        2,
                    )?;
                    self.write_items(result.to_content(), &direct, 2)?;
                    if !sub.is_empty() {
                        if direct.is_empty() {
                            result.do_indent(0)?;
                        }
                        result.to_content().write_all(&sub)?;
                    }
                    write!(result.to_content(), "}}")?;
                } else {
                    write!(result.to_content(), ";")?;
                }
            }

            Item::MixinDeclaration {
                ref name,
                ref args,
                ref body,
            } => scope.define_mixin(name, args, body),
            Item::MixinCall {
                ref name,
                ref args,
                ref body,
            } => {
                if let Some((m_args, m_body)) = scope.get_mixin(name) {
                    let mut scope =
                        m_args.eval(scope, &args.evaluate(scope, true)?)?;
                    scope.define_mixin(
                        "%%BODY%%",
                        &FormalArgs::default(),
                        body,
                    );
                    for item in m_body {
                        self.handle_root_item(
                            &item,
                            &mut scope,
                            file_context,
                            result,
                        )?;
                    }
                } else {
                    return Err(Error::S(format!(
                        "Unknown mixin {}({:?})",
                        name, args
                    )));
                }
            }
            Item::Content => {
                return Err(Error::S(
                    "@content not allowed in global context".into(),
                ));
            }

            Item::FunctionDeclaration { ref name, ref func } => {
                scope.define_function(name, func.clone());
            }
            Item::Return(_) => {
                return Err(Error::S(
                    "Return not allowed in global context".into(),
                ));
            }

            Item::IfStatement(ref cond, ref do_if, ref do_else) => {
                let cond = cond.evaluate(scope)?.is_true();
                let items = if cond { do_if } else { do_else };
                for item in items {
                    self.handle_root_item(item, scope, file_context, result)?;
                }
            }
            Item::Each(ref names, ref values, ref body) => {
                for value in values.evaluate(scope)?.iter_items() {
                    // TODO: No local sub-scope here?!?
                    scope.define_multi(names, &value);
                    for item in body {
                        self.handle_root_item(
                            item,
                            scope,
                            file_context,
                            result,
                        )?;
                    }
                }
            }
            Item::For {
                ref name,
                ref from,
                ref to,
                inclusive,
                ref body,
            } => {
                let from = from.evaluate(scope)?.integer_value()?;
                let to = to.evaluate(scope)?.integer_value()?;
                let to = if inclusive { to + 1 } else { to };
                for value in from..to {
                    let mut scope = ScopeImpl::sub(scope);
                    scope.define(name, &Value::scalar(value));
                    for item in body {
                        self.handle_root_item(
                            item,
                            &mut scope,
                            file_context,
                            result,
                        )?;
                    }
                }
            }
            Item::While(ref cond, ref body) => {
                let mut scope = ScopeImpl::sub(scope);
                while cond.evaluate(&scope)?.is_true() {
                    for item in body {
                        self.handle_root_item(
                            item,
                            &mut scope,
                            file_context,
                            result,
                        )?;
                    }
                }
            }

            Item::Rule(ref s, ref b) => {
                result.do_separate()?;
                self.write_rule(
                    s,
                    b,
                    result.to_content(),
                    scope,
                    file_context,
                    0,
                )?;
            }
            Item::NamespaceRule(..) => {
                panic!("Global namespaced property not allowed");
            }
            Item::Property(..) => {
                panic!("Global property not allowed");
            }
            Item::Comment(ref c) => {
                if !self.is_compressed() {
                    result.do_separate()?;
                    write!(result.to_content(), "/*{}*/", c)?;
                }
            }
            Item::None => (),
        }
        Ok(())
    }
    fn write_rule(
        &self,
        selectors: &Selectors,
        body: &[Item],
        out: &mut Write,
        scope: &mut Scope,
        file_context: &FileContext,
        indent: usize,
    ) -> Result<(), Error> {
        let selectors =
            eval_selectors(selectors, scope)?.inside(scope.get_selectors());
        let mut direct = Vec::new();
        let mut sub = Vec::new();
        self.handle_body(
            &mut direct,
            &mut sub,
            &mut ScopeImpl::sub_selectors(scope, selectors.clone()),
            body,
            file_context,
            indent,
        )?;
        if !direct.is_empty() {
            self.do_indent_no_lf(out, indent)?;
            if self.is_compressed() {
                write!(out, "{:#}{{", selectors)?;
            } else {
                write!(out, "{} {{", selectors)?;
            }
            self.write_items(out, &direct, indent + 2)?;
            write!(out, "}}")?;
            self.do_indent(out, 0)?;
        }
        out.write_all(&sub)?;
        Ok(())
    }

    fn handle_body(
        &self,
        direct: &mut Vec<CssBodyItem>,
        sub: &mut Write,
        scope: &mut Scope,
        body: &[Item],
        file_context: &FileContext,
        indent: usize,
    ) -> Result<(), Error> {
        for b in body {
            match *b {
                Item::Import(ref name) => {
                    let name = name.evaluate(scope)?;
                    if let Value::Literal(ref x, _) = name {
                        let (sub_context, file) =
                            file_context.file(x.as_ref());
                        let items = parse_scss_file(&file)?;
                        self.handle_body(
                            direct,
                            sub,
                            scope,
                            &items,
                            &sub_context,
                            0,
                        )?;
                    } else {
                        // TODO writeln!(direct, "@import {};", name)?;
                    }
                }
                Item::VariableDeclaration {
                    ref name,
                    ref val,
                    default,
                    global,
                } => {
                    let val = val.do_evaluate(scope, true)?;
                    if default {
                        scope.define_default(name, &val, global);
                    } else if global {
                        scope.define_global(name, &val);
                    } else {
                        scope.define(name, &val);
                    }
                }
                Item::AtRoot {
                    ref selectors,
                    ref body,
                } => {
                    let selectors = eval_selectors(selectors, scope)?
                        .with_backref(scope.get_selectors().one());
                    let mut s1 = vec![];
                    let mut s2 = vec![];
                    self.handle_body(
                        &mut s1,
                        &mut s2,
                        &mut ScopeImpl::sub_selectors(
                            scope,
                            selectors.clone(),
                        ),
                        body,
                        file_context,
                        indent,
                    )?;

                    if !s1.is_empty() {
                        if indent > 0 {
                            self.do_indent(sub, indent)?;
                        }
                        if self.is_compressed() {
                            write!(sub, "{:#}{{", selectors)?;
                        } else {
                            write!(sub, "{} {{", selectors)?;
                        }
                        self.write_items(sub, &s1, indent + 2)?;
                        write!(sub, "}}")?;
                        self.do_indent(sub, 0)?;
                    }
                    if !s2.is_empty() {
                        sub.write_all(&s2)?;
                    }
                }
                Item::AtRule {
                    ref name,
                    ref args,
                    ref body,
                } => {
                    write!(sub, "@{}", name)?;
                    let args = args.evaluate(scope)?;
                    if !args.is_null() {
                        write!(sub, " {}", args)?;
                    }
                    if let Some(ref body) = *body {
                        if self.is_compressed() {
                            write!(sub, "{{")?;
                        } else {
                            write!(sub, " {{")?;
                        }

                        let mut s1 = vec![];
                        let mut s2 = vec![];
                        self.handle_body(
                            &mut s1,
                            &mut s2,
                            &mut ScopeImpl::sub(scope),
                            body,
                            file_context,
                            2,
                        )?;

                        if !s1.is_empty() {
                            self.do_indent(sub, 2)?;
                            if self.is_compressed() {
                                write!(sub, "{:#}{{", scope.get_selectors())?;
                            } else {
                                write!(sub, "{} {{", scope.get_selectors())?;
                            }
                            self.write_items(sub, &s1, 4)?;
                            write!(sub, "}}")?;
                        }
                        self.do_indent(sub, 0)?;
                        if !s2.is_empty() {
                            sub.write_all(&s2)?;
                        }
                        write!(sub, "}}")?;
                        self.do_indent(sub, 0)?;
                    } else {
                        write!(sub, ";")?;
                    }
                }

                Item::MixinDeclaration {
                    ref name,
                    ref args,
                    ref body,
                } => {
                    scope.define_mixin(name, args, body);
                }
                Item::MixinCall {
                    ref name,
                    ref args,
                    ref body,
                } => {
                    if let Some((m_args, m_body)) = scope.get_mixin(name) {
                        let mut argscope = m_args
                            .eval(scope, &args.evaluate(scope, true)?)?;
                        argscope.define_mixin(
                            "%%BODY%%",
                            &FormalArgs::default(),
                            body,
                        );
                        self.handle_body(
                            direct,
                            sub,
                            &mut argscope,
                            &m_body,
                            file_context,
                            indent,
                        )?;
                    } else {
                        direct.push(CssBodyItem::Comment(format!(
                            "Unknown mixin {}({:?})",
                            name, args
                        )));
                    }
                }
                Item::Content => {
                    if let Some((_args, m_body)) = scope.get_mixin("%%BODY%%")
                    {
                        self.handle_body(
                            direct,
                            sub,
                            scope,
                            &m_body,
                            file_context,
                            indent,
                        )?;
                    }
                }

                Item::FunctionDeclaration { ref name, ref func } => {
                    scope.define_function(name, func.clone());
                }
                Item::Return(_) => {
                    panic!("Return not allowed in plain context");
                }

                Item::IfStatement(ref cond, ref do_if, ref do_else) => {
                    let cond = cond.evaluate(scope)?.is_true();
                    let items = if cond { do_if } else { do_else };
                    self.handle_body(
                        direct,
                        sub,
                        &mut ScopeImpl::sub(scope),
                        items,
                        file_context,
                        0,
                    )?;
                }
                Item::Each(ref names, ref values, ref body) => {
                    for value in values.evaluate(scope)?.iter_items() {
                        let mut scope = ScopeImpl::sub(scope);
                        scope.define_multi(&names, &value);
                        self.handle_body(
                            direct,
                            sub,
                            &mut scope,
                            body,
                            file_context,
                            0,
                        )?;
                    }
                }
                Item::For {
                    ref name,
                    ref from,
                    ref to,
                    inclusive,
                    ref body,
                } => {
                    let from = from.evaluate(scope)?.integer_value()?;
                    let to = to.evaluate(scope)?.integer_value()?;
                    let to = if inclusive { to + 1 } else { to };
                    for value in from..to {
                        let mut scope = ScopeImpl::sub(scope);
                        scope.define(name, &Value::scalar(value));
                        self.handle_body(
                            direct,
                            sub,
                            &mut scope,
                            body,
                            file_context,
                            0,
                        )?;
                    }
                }
                Item::While(ref cond, ref body) => {
                    let mut scope = ScopeImpl::sub(scope);
                    while cond.evaluate(&scope)?.is_true() {
                        self.handle_body(
                            direct,
                            sub,
                            &mut scope,
                            body,
                            file_context,
                            0,
                        )?;
                    }
                }

                Item::Rule(ref s, ref b) => {
                    self.write_rule(s, b, sub, scope, file_context, indent)?;
                }
                Item::NamespaceRule(ref name, ref value, ref body) => {
                    let value = value.evaluate(scope)?;
                    let (name, _quotes) = name.evaluate(scope)?;
                    if !value.is_null() {
                        direct
                            .push(CssBodyItem::Property(name.clone(), value));
                    }
                    let mut t = Vec::new();
                    self.handle_body(
                        &mut t,
                        sub,
                        scope,
                        body,
                        file_context,
                        indent,
                    )?;
                    for item in t {
                        direct.push(match item {
                            CssBodyItem::Property(n, v) => {
                                CssBodyItem::Property(
                                    format!("{}-{}", name, n),
                                    v,
                                )
                            }
                            c => c,
                        })
                    }
                }
                Item::Property(ref name, ref value) => {
                    let v = value.evaluate(scope)?;
                    if !v.is_null() {
                        let (name, _q) = name.evaluate(scope)?;
                        direct.push(CssBodyItem::Property(name, v));
                    }
                }
                Item::Comment(ref c) => {
                    if !self.is_compressed() {
                        direct.push(CssBodyItem::Comment(c.clone()));
                    }
                }
                Item::None => (),
            }
        }
        Ok(())
    }

    fn write_items(
        &self,
        out: &mut Write,
        items: &[CssBodyItem],
        indent: usize,
    ) -> Result<(), Error> {
        if !items.is_empty() {
            let mut buf = Vec::new();
            for item in items {
                self.do_indent(&mut buf, indent)?;
                if self.is_compressed() {
                    write!(buf, "{:#}", item)?;
                } else {
                    write!(buf, "{}", item)?;
                }
            }
            if self.is_compressed() && buf.last() == Some(&b';') {
                buf.pop();
            }
            out.write_all(&buf)?;
            self.do_indent(out, indent - 2)?;
        }
        Ok(())
    }

    fn do_indent(&self, out: &mut Write, steps: usize) -> Result<(), Error> {
        if !self.is_compressed() {
            writeln!(out)?;
            for _i in 0..steps {
                write!(out, " ")?;
            }
        }
        Ok(())
    }
    fn do_indent_no_lf(
        &self,
        out: &mut Write,
        steps: usize,
    ) -> Result<(), Error> {
        if !self.is_compressed() {
            for _i in 0..steps {
                write!(out, " ")?;
            }
        }
        Ok(())
    }

    fn is_compressed(&self) -> bool {
        self == &OutputStyle::Compressed
    }
}

fn eval_selectors(s: &Selectors, scope: &Scope) -> Result<Selectors, Error> {
    let s = Selectors::new(
        s.s.iter()
            .map(|s| -> Result<Selector, Error> {
                Ok(Selector(
                    s.0.iter()
                        .map(|sp| -> Result<SelectorPart, Error> {
                            match *sp {
                                SelectorPart::Attribute {
                                    ref name,
                                    ref op,
                                    ref val,
                                    ref modifier,
                                } => {
                                    // A bit strange, but tests seems to
                                    // suggest this.
                                    let val = if modifier.is_some() {
                                        val.evaluate_opt_unquote(scope)?
                                    } else {
                                        val.evaluate2(scope)?
                                    };
                                    Ok(SelectorPart::Attribute {
                                        name: name.evaluate2(scope)?,
                                        op: op.clone(),
                                        val,
                                        modifier: modifier.clone(),
                                    })
                                }
                                SelectorPart::Simple(ref v) => Ok(
                                    SelectorPart::Simple(v.evaluate2(scope)?),
                                ),
                                SelectorPart::Pseudo {
                                    ref name,
                                    ref arg,
                                } => {
                                    let evaluated_arg = match &arg {
                                        Some(ref a) => {
                                            Some(eval_selectors(a, scope)?)
                                        }
                                        None => None,
                                    };
                                    Ok(SelectorPart::Pseudo {
                                        name: name.evaluate2(scope)?,
                                        arg: evaluated_arg,
                                    })
                                }
                                SelectorPart::PseudoElement {
                                    ref name,
                                    ref arg,
                                } => {
                                    let evaluated_arg = match &arg {
                                        Some(ref a) => {
                                            Some(eval_selectors(a, scope)?)
                                        }
                                        None => None,
                                    };
                                    Ok(SelectorPart::PseudoElement {
                                        name: name.evaluate2(scope)?,
                                        arg: evaluated_arg,
                                    })
                                }
                                ref sp => Ok(sp.clone()),
                            }
                        })
                        .collect::<Result<Vec<_>, Error>>()?,
                ))
            })
            .collect::<Result<Vec<_>, Error>>()?,
    );
    // The "simple" parts we get from evaluating interpolations may
    // contain high-level selector separators (i.e. ","), so we need to
    // parse the selectors again, from a string representation.
    use crate::parser::selectors::selectors;
    Ok(selectors(format!("{} ", s).as_bytes())?.1)
}

struct CssWriter {
    imports: Vec<u8>,
    contents: Vec<u8>,
    style: OutputStyle,
    separate: bool,
}

impl CssWriter {
    fn new(style: OutputStyle) -> Self {
        CssWriter {
            imports: Vec::new(),
            contents: Vec::new(),
            style,
            separate: false,
        }
    }
    fn get_result(self) -> Result<Vec<u8>, Error> {
        let mut result = vec![];
        let compressed = self.is_compressed();
        if !self.imports.is_ascii() || !self.contents.is_ascii() {
            if compressed {
                // U+FEFF is byte order mark, used to show encoding.
                result.extend_from_slice("\u{feff}".as_bytes());
            } else {
                result.extend_from_slice(b"@charset \"UTF-8\";\n");
            }
        }
        result.extend(self.imports);
        result.extend(self.contents);
        if compressed && result.last() == Some(&b';') {
            result.pop();
        }
        if result.last().unwrap_or(&b'\n') != &b'\n' {
            writeln!(&mut result)?;
        }
        Ok(result)
    }

    fn to_imports(&mut self) -> &mut Write {
        &mut self.imports
    }
    fn to_content(&mut self) -> &mut Write {
        &mut self.contents
    }
    fn do_separate(&mut self) -> Result<(), Error> {
        if self.separate {
            self.do_indent(0)?;
        } else {
            self.separate = true;
        }
        Ok(())
    }
    fn do_indent(&mut self, steps: usize) -> Result<(), Error> {
        if !self.is_compressed() {
            if !self.contents.is_empty() {
                writeln!(self.contents)?;
            }
            for _i in 0..steps {
                write!(self.contents, " ")?;
            }
        }
        Ok(())
    }
    fn is_compressed(&self) -> bool {
        self.style == OutputStyle::Compressed
    }
}

enum CssBodyItem {
    Property(String, Value),
    Comment(String),
}

impl fmt::Display for CssBodyItem {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CssBodyItem::Property(ref name, ref val) => {
                if out.alternate() {
                    write!(out, "{}:{:#};", name, val)
                } else {
                    write!(out, "{}: {};", name, val)
                }
            }
            CssBodyItem::Comment(ref c) => write!(out, "/*{}*/", c),
        }
    }
}
