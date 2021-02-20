use super::Format;
use crate::css::{BodyItem, Rule};
use crate::error::Error;
use crate::file_context::FileContext;
use crate::parser::parse_imported_scss_file;
use crate::sass::{FormalArgs, Item, SassString};
use crate::selectors::Selectors;
use crate::value::ValueRange;
use crate::variablescope::{Scope, ScopeImpl};
use std::fmt;
use std::io::Write;
use std::str::FromStr;

/// Selected target format.
/// Only formats that are variants of this type are supported by rsass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    /// The expanded format, nice readable css.
    Expanded,
    /// The compressed format, saves download size.
    Compressed,
    /// Special format used by the inspect(value) sass function
    Introspection,
}

impl fmt::Display for Style {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        out.write_str(match self {
            Style::Compressed => "compressed",
            Style::Expanded => "expanded",
            Style::Introspection => "introspection",
        })
    }
}

/// Get an output style from its name.
impl FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "compressed" => Ok(Style::Compressed),
            "expanded" => Ok(Style::Expanded),
            s => Err(format!("Output style {:?} not supported", s)),
        }
    }
}

static FORMAT_NAMES: [&str; 2] = ["Compressed", "Expanded"];

impl Style {
    /// Get the names of the supported output styles.
    pub fn variants() -> &'static [&'static str] {
        &FORMAT_NAMES
    }
}

impl Format {
    /// Write a slice of sass items in this format.
    /// The `file_context` is needed if there are `@import` statements
    /// in the sass file.
    pub fn write_root(
        &self,
        items: &[Item],
        globals: &mut dyn Scope,
        file_context: &impl FileContext,
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
        scope: &mut dyn Scope,
        file_context: &impl FileContext,
        result: &mut CssWriter,
    ) -> Result<(), Error> {
        match *item {
            Item::Use(ref name, ref as_n) => do_use(scope, name, as_n)?,
            Item::Import(ref names, ref args, ref pos) => {
                'name: for name in names {
                    if args.is_null() {
                        let (x, _q) = name.evaluate(scope)?;
                        if let Some((sub_context, path, mut file)) =
                            file_context.find_file(&x)?
                        {
                            for item in parse_imported_scss_file(
                                &mut file,
                                &path,
                                pos.clone(),
                            )? {
                                self.handle_root_item(
                                    &item,
                                    scope,
                                    &sub_context,
                                    result,
                                )?;
                            }
                            continue 'name;
                        }
                    }
                    result.add_import(
                        name.evaluate2(scope)?,
                        args.evaluate(scope)?,
                    )?;
                }
            }
            Item::VariableDeclaration {
                ref name,
                ref val,
                default,
                global,
            } => {
                let val = val.do_evaluate(scope, true)?;
                scope.set_variable(name.into(), val, default, global);
            }
            Item::AtRoot(ref selectors, ref body) => {
                let selectors = selectors
                    .eval(scope)?
                    .with_backref(scope.get_selectors().one());
                let mut rule = Rule::new(selectors.clone());
                let mut s2 = CssBuf::new(*self, 0);
                self.handle_body(
                    rule.mut_body(),
                    &mut s2,
                    &mut ScopeImpl::sub_selectors(scope, selectors),
                    body,
                    file_context,
                )?;
                result.to_content().write_rule(&rule)?;
                result.to_content().join(s2);
            }
            Item::AtRule {
                ref name,
                ref args,
                ref body,
            } => {
                result.do_separate();
                let args = args.evaluate(scope)?;
                write!(result.to_content().buf, "@{}", name)?;
                if !args.is_null() {
                    write!(
                        result.to_content().buf,
                        " {}",
                        args.format(*self),
                    )?;
                }
                if let Some(ref body) = *body {
                    if self.is_compressed() {
                        result.to_content().get_mut_buf().push(b'{');
                    } else {
                        result.to_content().get_mut_buf().extend(b" {");
                    }
                    let mut direct = vec![];
                    let mut sub = CssBuf::new(scope.get_format(), 2);
                    self.handle_body(
                        &mut direct,
                        &mut sub,
                        &mut ScopeImpl::sub(scope),
                        body,
                        file_context,
                    )?;
                    if !direct.is_empty() {
                        let mut buf = Vec::new();
                        for item in &direct {
                            self.do_indent(&mut buf, 2)?;
                            item.write(&mut buf, *self)?;
                        }
                        if self.is_compressed() && buf.last() == Some(&b';') {
                            buf.pop();
                        }
                        result.to_content().buf.write_all(&buf)?;
                        result.to_content().do_indent();
                    }
                    if !sub.is_empty() {
                        if direct.is_empty() {
                            result.to_content().do_indent();
                        }
                        result.to_content().join(sub);
                    }
                    result.to_content().get_mut_buf().push(b'}');
                } else {
                    result.to_content().get_mut_buf().push(b';');
                }
            }

            Item::MixinDeclaration(ref name, ref args, ref body) => {
                scope.define_mixin(name, args, body)
            }
            Item::MixinCall(ref name, ref args, ref body) => {
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

            Item::FunctionDeclaration(ref name, ref func) => {
                scope.define_function(name.into(), func.clone());
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
                let range = ValueRange::new(
                    from.evaluate(scope)?,
                    to.evaluate(scope)?,
                    inclusive,
                )?;
                for value in range {
                    let mut scope = ScopeImpl::sub(scope);
                    scope.define(name.clone(), &value);
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
            Item::Debug(ref value) => {
                eprintln!("DEBUG: {}", value.evaluate(scope)?.format(*self));
            }
            Item::Warn(ref value) => {
                eprintln!(
                    "WARNING: {}",
                    value.evaluate(scope)?.format(*self)
                );
            }
            Item::Error(ref value) => {
                return Err(Error::S(format!(
                    "Error: {}",
                    value.evaluate(scope)?.format(*self)
                )));
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
                result.do_separate();
                self.write_rule(
                    s,
                    b,
                    result.to_content(),
                    scope,
                    file_context,
                )?;
            }
            Item::NamespaceRule(..) => {
                return Err(Error::S(
                    "Global namespaced property not allowed".into(),
                ));
            }
            Item::Property(..) => {
                return Err(Error::S("Global property not allowed".into()));
            }
            Item::Comment(ref c) => {
                if !self.is_compressed() {
                    result.do_separate();
                    let (c, _q) = c.evaluate(scope)?;
                    write!(result.to_content().get_mut_buf(), "/*{}*/", c)?;
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
        out: &mut CssBuf,
        scope: &mut dyn Scope,
        file_context: &impl FileContext,
    ) -> Result<(), Error> {
        let selectors = selectors.eval(scope)?.inside(scope.get_selectors());
        let mut rule = Rule::new(selectors.clone());
        let mut sub = CssBuf::new(*self, out.indent);
        self.handle_body(
            rule.mut_body(),
            &mut sub,
            &mut ScopeImpl::sub_selectors(scope, selectors),
            body,
            file_context,
        )?;
        out.write_rule(&rule)?;
        out.join(sub);
        Ok(())
    }

    fn handle_body(
        &self,
        direct: &mut Vec<BodyItem>,
        sub: &mut CssBuf,
        scope: &mut dyn Scope,
        body: &[Item],
        file_context: &impl FileContext,
    ) -> Result<(), Error> {
        for b in body {
            match *b {
                Item::Use(ref name, ref as_n) => do_use(scope, name, as_n)?,
                Item::Import(ref names, ref args, ref pos) => {
                    if args.is_null() {
                        for name in names {
                            let (x, _q) = name.evaluate(scope)?;
                            if let Some((sub_context, path, mut file)) =
                                file_context.find_file(x.as_ref())?
                            {
                                let items = parse_imported_scss_file(
                                    &mut file,
                                    &path,
                                    pos.clone(),
                                )?;
                                self.handle_body(
                                    direct,
                                    sub,
                                    scope,
                                    &items,
                                    &sub_context,
                                )?;
                            } else {
                                sub.do_indent();
                                write!(
                                    sub.get_mut_buf(), // TODO:  Should be topmost!
                                    "@import {};{}",
                                    name.evaluate2(scope)?,
                                    if self.is_compressed() {
                                        ""
                                    } else {
                                        "\n"
                                    }
                                )?;
                            }
                        }
                    } else {
                        for name in names {
                            sub.do_indent();
                            write!(
                                sub.get_mut_buf(), // TODO:  Should be topmost!
                                "@import {} {};{}",
                                name.evaluate2(scope)?,
                                args.evaluate(scope)?.format(*self),
                                if self.is_compressed() { "" } else { "\n" }
                            )?;
                        }
                    }
                }
                Item::VariableDeclaration {
                    ref name,
                    ref val,
                    default,
                    global,
                } => {
                    let val = val.do_evaluate(scope, true)?;
                    scope.set_variable(name.into(), val, default, global);
                }
                Item::AtRoot(ref selectors, ref body) => {
                    let selectors = selectors
                        .eval(scope)?
                        .with_backref(scope.get_selectors().one());
                    let mut rule = Rule::new(selectors.clone());
                    let mut sub2 = CssBuf::new(*self, sub.indent);
                    self.handle_body(
                        rule.mut_body(),
                        &mut sub2,
                        &mut ScopeImpl::sub_selectors(scope, selectors),
                        body,
                        file_context,
                    )?;
                    sub.write_rule(&rule)?;
                    sub.join(sub2);
                }
                Item::AtRule {
                    ref name,
                    ref args,
                    ref body,
                } => {
                    write!(sub.get_mut_buf(), "@{}", name)?;
                    let args = args.evaluate(scope)?;
                    if !args.is_null() {
                        write!(sub.get_mut_buf(), " {}", args.format(*self))?;
                    }
                    if let Some(ref body) = *body {
                        if self.is_compressed() {
                            sub.get_mut_buf().push(b'{');
                        } else {
                            sub.get_mut_buf().extend(b" {");
                        }

                        let mut rule =
                            Rule::new(scope.get_selectors().clone());
                        let mut sub2 = CssBuf::new(*self, 2);
                        self.handle_body(
                            rule.mut_body(),
                            &mut sub2,
                            &mut ScopeImpl::sub(scope),
                            body,
                            file_context,
                        )?;
                        sub.do_indent();
                        rule.write(sub.get_mut_buf(), *self, 2)?;
                        sub.join(sub2);
                        sub.get_mut_buf().push(b'}');
                        sub.do_indent();
                    } else {
                        sub.get_mut_buf().push(b';');
                    }
                }

                Item::MixinDeclaration(ref name, ref args, ref body) => {
                    scope.define_mixin(name, args, body);
                }
                Item::MixinCall(ref name, ref args, ref body) => {
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
                        )?;
                    } else {
                        direct.push(BodyItem::Comment(format!(
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
                        )?;
                    }
                }

                Item::FunctionDeclaration(ref name, ref func) => {
                    scope.define_function(name.into(), func.clone());
                }
                Item::Return(_) => {
                    return Err(Error::S(
                        "Return not allowed in plain context".into(),
                    ));
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
                    let range = ValueRange::new(
                        from.evaluate(scope)?,
                        to.evaluate(scope)?,
                        inclusive,
                    )?;
                    for value in range {
                        let mut scope = ScopeImpl::sub(scope);
                        scope.define(name.clone(), &value);
                        self.handle_body(
                            direct,
                            sub,
                            &mut scope,
                            body,
                            file_context,
                        )?;
                    }
                }
                Item::Debug(ref value) => {
                    eprintln!(
                        "DEBUG: {}",
                        value.evaluate(scope)?.format(*self)
                    );
                }
                Item::Warn(ref value) => {
                    eprintln!(
                        "WARNING: {}",
                        value.evaluate(scope)?.format(*self)
                    );
                }
                Item::Error(ref value) => {
                    return Err(Error::S(format!(
                        "Error: {}",
                        value.evaluate(scope)?.format(*self),
                    )));
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
                        )?;
                    }
                }

                Item::Rule(ref s, ref b) => {
                    self.write_rule(s, b, sub, scope, file_context)?;
                }
                Item::NamespaceRule(ref name, ref value, ref body) => {
                    let value = value.evaluate(scope)?;
                    let (name, _quotes) = name.evaluate(scope)?;
                    if !value.is_null() {
                        direct.push(BodyItem::Property(name.clone(), value));
                    }
                    let mut t = Vec::new();
                    self.handle_body(&mut t, sub, scope, body, file_context)?;
                    for item in t {
                        direct.push(match item {
                            BodyItem::Property(n, v) => BodyItem::Property(
                                format!("{}-{}", name, n),
                                v,
                            ),
                            c => c,
                        })
                    }
                }
                Item::Property(ref name, ref value) => {
                    let v = value.evaluate(scope)?;
                    if !v.is_null() {
                        let (name, _q) = name.evaluate(scope)?;
                        direct.push(BodyItem::Property(name, v));
                    }
                }
                Item::Comment(ref c) => {
                    if !self.is_compressed() {
                        let (c, _q) = c.evaluate(scope)?;
                        direct.push(BodyItem::Comment(c));
                    }
                }
                Item::None => (),
            }
        }
        Ok(())
    }

    fn do_indent(
        &self,
        out: &mut dyn Write,
        steps: usize,
    ) -> Result<(), Error> {
        Ok(out.write_all(self.get_indent(steps).as_ref())?)
    }
    /// Get a newline followed by len spaces, unles self is compressed.
    pub fn get_indent(&self, len: usize) -> &'static str {
        static INDENT: &str = "\n                                                                                ";
        if self.is_compressed() {
            ""
        } else {
            &INDENT[..(len + 1)]
        }
    }
}

use crate::sass::UseAs;
fn do_use(
    scope: &mut dyn Scope,
    name: &SassString,
    as_name: &UseAs,
) -> Result<(), Error> {
    use crate::functions::get_global_module;
    let name = name.evaluate(scope)?.0;
    if let Some(module) = get_global_module(&name) {
        match as_name {
            UseAs::KeepName => {
                let name =
                    name.rfind(':').map(|i| &name[i + 1..]).unwrap_or(&name);
                scope.define_module(name.into(), module);
            }
            UseAs::Star => {
                for (name, function) in module.functions() {
                    scope.define_function(name.clone(), function.clone());
                }
                for (name, value) in module.variables() {
                    scope.define(name.clone(), value);
                }
            }
            UseAs::Name(name) => {
                let name = name.evaluate(scope)?.0;
                scope.define_module(name.into(), module);
            }
        }
        Ok(())
    } else {
        Err(Error::S(format!("Module {:?} not found", name)))
    }
}

struct CssWriter {
    imports: Vec<u8>,
    contents: CssBuf,
    format: Format,
    separate: bool,
}

impl CssWriter {
    fn new(format: Format) -> Self {
        CssWriter {
            imports: Vec::new(),
            contents: CssBuf::new(format, 0),
            format,
            separate: false,
        }
    }
    fn get_result(self) -> Result<Vec<u8>, Error> {
        let mut result = vec![];
        let compressed = self.format.is_compressed();
        if !self.imports.is_ascii() || !self.contents.is_ascii() {
            if compressed {
                // U+FEFF is byte order mark, used to show encoding.
                result.extend_from_slice("\u{feff}".as_bytes());
            } else {
                result.extend_from_slice(b"@charset \"UTF-8\";\n");
            }
        }
        result.extend(self.imports);
        result.extend(self.contents.buf);
        if compressed && result.last() == Some(&b';') {
            result.pop();
        }
        if result.last().unwrap_or(&b'\n') != &b'\n' {
            writeln!(&mut result)?;
        }
        Ok(result)
    }

    pub fn add_import(
        &mut self,
        name: SassString,
        args: crate::css::Value,
    ) -> Result<(), Error> {
        write!(&mut self.imports, "@import {}", name)?;
        if !args.is_null() {
            write!(&mut self.imports, " {}", args.format(self.format))?;
        }
        self.imports.extend(if self.format.is_compressed() {
            &b";"[..]
        } else {
            &b";\n"[..]
        });
        Ok(())
    }

    fn to_content(&mut self) -> &mut CssBuf {
        &mut self.contents
    }
    fn do_separate(&mut self) {
        if self.separate {
            if !self.format.is_compressed() && !self.contents.is_empty() {
                self.contents.buf.push(b'\n');
            }
        } else {
            self.separate = true;
        }
    }
}

struct CssBuf {
    buf: Vec<u8>,
    format: Format,
    indent: usize,
}

impl CssBuf {
    fn new(format: Format, indent: usize) -> CssBuf {
        CssBuf {
            buf: Vec::new(),
            format,
            indent,
        }
    }
    fn write_rule(&mut self, rule: &Rule) -> std::io::Result<()> {
        rule.write(&mut self.buf, self.format, self.indent)
    }
    fn do_indent(&mut self) {
        self.buf
            .extend(self.format.get_indent(self.indent).as_bytes())
    }

    fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    fn is_ascii(&self) -> bool {
        self.buf.is_ascii()
    }
    // TODO: Should this exist?
    fn get_mut_buf(&mut self) -> &mut Vec<u8> {
        &mut self.buf
    }

    fn join(&mut self, sub: Self) {
        self.buf.extend(sub.buf);
    }
}
