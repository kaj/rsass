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
        let mut head = CssBuf::new(*self, 0);
        let mut body = CssBuf::new(*self, 0);
        handle_body(
            items,
            &mut head,
            None,
            &mut body,
            globals,
            file_context,
        )?;
        let mut result = vec![];
        let compressed = self.is_compressed();
        if !head.is_ascii() || !body.is_ascii() {
            if compressed {
                // U+FEFF is byte order mark, used to show encoding.
                result.extend_from_slice("\u{feff}".as_bytes());
            } else {
                result.extend_from_slice(b"@charset \"UTF-8\";\n");
            }
        }
        result.extend(head.buf);
        result.extend(body.buf);
        if compressed && result.last() == Some(&b';') {
            result.pop();
        }
        if result.last().unwrap_or(&b'\n') != &b'\n' {
            writeln!(&mut result)?;
        }
        Ok(result)
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

fn handle_body(
    items: &[Item],
    head: &mut CssBuf,
    rule: Option<&mut Rule>,
    buf: &mut CssBuf,
    scope: &mut dyn Scope,
    file_context: &impl FileContext,
) -> Result<(), Error> {
    let mut rule = rule;
    for b in items {
        handle_item(b, head, rule.as_deref_mut(), buf, scope, file_context)?;
    }
    Ok(())
}

fn handle_item(
    item: &Item,
    head: &mut CssBuf,
    rule: Option<&mut Rule>,
    buf: &mut CssBuf,
    scope: &mut dyn Scope,
    file_context: &impl FileContext,
) -> Result<(), Error> {
    let format = scope.get_format();
    match item {
        Item::Use(ref name, ref as_n) => {
            use crate::functions::get_global_module;
            use crate::sass::UseAs;
            let name = name.evaluate(scope)?.0;
            if let Some(module) = get_global_module(&name) {
                match as_n {
                    UseAs::KeepName => {
                        let name = name
                            .rfind(':')
                            .map(|i| &name[i + 1..])
                            .unwrap_or(&name);
                        scope.define_module(name.into(), module);
                    }
                    UseAs::Star => {
                        for (name, function) in module.functions() {
                            scope.define_function(
                                name.clone(),
                                function.clone(),
                            );
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
            } else {
                return Err(Error::S(format!("Module {:?} not found", name)));
            }
        }
        Item::Import(ref names, ref args, ref pos) => {
            let mut rule = rule;
            'name: for name in names {
                if args.is_null() {
                    let (x, _q) = name.evaluate(scope)?;
                    if let Some((sub_context, path, mut file)) =
                        file_context.find_file(&x)?
                    {
                        let items = parse_imported_scss_file(
                            &mut file,
                            &path,
                            pos.clone(),
                        )?;
                        handle_body(
                            &items,
                            head,
                            rule.as_deref_mut(),
                            buf,
                            scope,
                            &sub_context,
                        )?;
                        continue 'name;
                    }
                }
                if buf.indent == 0 {
                    head.add_import(
                        name.evaluate2(scope)?,
                        args.evaluate(scope)?,
                    )?;
                } else {
                    buf.add_import(
                        name.evaluate2(scope)?,
                        args.evaluate(scope)?,
                    )?;
                }
            }
        }
        Item::AtRoot(ref selectors, ref body) => {
            let selectors = selectors
                .eval(scope)?
                .with_backref(scope.get_selectors().one());
            let mut rule = Rule::new(selectors.clone());
            let mut sub = CssBuf::new(format, buf.indent);
            handle_body(
                body,
                head,
                Some(&mut rule),
                &mut sub,
                &mut ScopeImpl::sub_selectors(scope, selectors),
                file_context,
            )?;
            buf.write_rule(&rule)?;
            buf.join(sub);
        }
        Item::AtRule {
            ref name,
            ref args,
            ref body,
        } => {
            buf.do_separate();
            write!(buf.buf, "@{}", name)?;
            let args = args.evaluate(scope)?;
            if !args.is_null() {
                write!(buf.buf, " {}", args.format(format))?;
            }
            if let Some(ref body) = *body {
                if format.is_compressed() {
                    buf.buf.push(b'{');
                } else {
                    buf.buf.extend(b" {");
                }
                let selectors = scope.get_selectors().clone();
                let has_selectors = selectors != Selectors::root();
                let mut rule = Rule::new(selectors);
                let mut sub = CssBuf::new(format, buf.indent + 2);
                handle_body(
                    body,
                    head,
                    Some(&mut rule),
                    &mut sub,
                    &mut ScopeImpl::sub(scope),
                    file_context,
                )?;
                let b0 = buf.buf.len();
                if has_selectors {
                    buf.do_indent();
                    buf.indent += 2;
                    buf.write_rule(&rule)?;
                    buf.indent -= 2;
                    if !sub.is_empty() {
                        buf.do_indent();
                    }
                } else {
                    let mut t = CssBuf::new(format, sub.indent);
                    for item in rule.body {
                        t.write_body_item(&item)?;
                    }
                    if format.is_compressed() && t.buf.last() == Some(&b';') {
                        t.buf.pop();
                    }
                    buf.join(t);
                    if !sub.is_empty() {
                        buf.do_indent();
                    }
                }
                if !sub.is_empty() {
                    buf.join(sub);
                }
                if buf.buf.len() != b0 && buf.buf.last() != Some(&b'\n') {
                    buf.do_indent();
                }
                buf.buf.push(b'}');
                buf.do_indent();
            } else {
                buf.buf.push(b';');
            }
        }

        Item::VariableDeclaration {
            ref name,
            ref val,
            default,
            global,
        } => {
            let val = val.do_evaluate(scope, true)?;
            scope.set_variable(name.into(), val, *default, *global);
        }

        Item::FunctionDeclaration(ref name, ref func) => {
            scope.define_function(name.into(), func.clone());
        }
        Item::Return(_) => {
            return Err(Error::S(
                "Return not allowed in plain context".into(),
            ));
        }

        Item::MixinDeclaration(ref name, ref args, ref body) => {
            scope.define_mixin(name, args, body)
        }
        Item::MixinCall(ref name, ref args, ref body) => {
            if let Some((m_args, m_body)) = scope.get_mixin(name) {
                let mut scope =
                    m_args.eval(scope, &args.evaluate(scope, true)?)?;
                scope.define_mixin("%%BODY%%", &FormalArgs::default(), body);
                handle_body(
                    &m_body,
                    head,
                    rule,
                    buf,
                    &mut scope,
                    file_context,
                )?;
            } else {
                return Err(Error::S(format!(
                    "Unknown mixin {}({:?})",
                    name, args
                )));
            }
        }
        Item::Content => {
            if let Some(rule) = rule {
                if let Some((_args, m_body)) = scope.get_mixin("%%BODY%%") {
                    handle_body(
                        &m_body,
                        head,
                        Some(rule),
                        buf,
                        scope,
                        file_context,
                    )?;
                }
            } else {
                return Err(Error::S(
                    "@content not allowed in global context".into(),
                ));
            }
        }

        Item::IfStatement(ref cond, ref do_if, ref do_else) => {
            let cond = cond.evaluate(scope)?.is_true();
            let items = if cond { do_if } else { do_else };
            handle_body(items, head, rule, buf, scope, file_context)?;
        }
        Item::Each(ref names, ref values, ref body) => {
            let mut rule = rule;
            let pushed = scope.store_local_values(names);
            for value in values.evaluate(scope)?.iter_items() {
                scope.define_multi(&names, &value);
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    scope,
                    file_context,
                )?;
            }
            scope.restore_local_values(pushed);
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
                *inclusive,
            )?;
            let mut rule = rule;
            for value in range {
                let mut scope = ScopeImpl::sub(scope);
                scope.define(name.clone(), &value);
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    &mut scope,
                    file_context,
                )?;
            }
        }
        Item::While(ref cond, ref body) => {
            let mut rule = rule;
            let mut scope = ScopeImpl::sub(scope);
            while cond.evaluate(&scope)?.is_true() {
                handle_body(
                    body,
                    head,
                    rule.as_deref_mut(),
                    buf,
                    &mut scope,
                    file_context,
                )?;
            }
        }

        Item::Debug(ref value) => {
            eprintln!("DEBUG: {}", value.evaluate(scope)?.format(format));
        }
        Item::Warn(ref value) => {
            eprintln!("WARNING: {}", value.evaluate(scope)?.format(format));
        }
        Item::Error(ref value) => {
            return Err(Error::S(format!(
                "Error: {}",
                value.evaluate(scope)?.format(format)
            )));
        }

        Item::Rule(ref selectors, ref body) => {
            if rule.is_none() {
                buf.do_separate();
            }
            let selectors =
                selectors.eval(scope)?.inside(scope.get_selectors());
            let mut rule = Rule::new(selectors.clone());
            let mut sub = CssBuf::new(format, buf.indent);
            handle_body(
                body,
                head,
                Some(&mut rule),
                &mut sub,
                &mut ScopeImpl::sub_selectors(scope, selectors),
                file_context,
            )?;
            buf.write_rule(&rule)?;
            buf.join(sub);
        }
        Item::Property(ref name, ref value) => {
            if let Some(rule) = rule {
                let v = value.evaluate(scope)?;
                if !v.is_null() {
                    let (name, _q) = name.evaluate(scope)?;
                    rule.push(BodyItem::Property(name, v));
                }
            } else {
                return Err(Error::S("Global property not allowed".into()));
            }
        }
        Item::NamespaceRule(ref name, ref value, ref body) => {
            if let Some(rule) = rule {
                let value = value.evaluate(scope)?;
                let (name, _quotes) = name.evaluate(scope)?;
                if !value.is_null() {
                    rule.push(BodyItem::Property(name.clone(), value));
                }
                let mut t = Rule::new(Selectors::root());
                let mut sub = CssBuf::new(format, 0);
                handle_body(
                    body,
                    head,
                    Some(&mut t),
                    &mut sub,
                    scope,
                    file_context,
                )?;
                for item in t.body {
                    rule.push(match item {
                        BodyItem::Property(n, v) => {
                            BodyItem::Property(format!("{}-{}", name, n), v)
                        }
                        c => c,
                    })
                }
                assert!(sub.is_empty());
            } else {
                return Err(Error::S(
                    "Global namespaced property not allowed".into(),
                ));
            }
        }
        Item::Comment(ref c) => {
            if !format.is_compressed() {
                if let Some(rule) = rule {
                    let (c, _q) = c.evaluate(scope)?;
                    rule.push(BodyItem::Comment(c));
                } else {
                    buf.do_separate();
                    let (c, _q) = c.evaluate(scope)?;
                    write!(buf.buf, "/*{}*/", c)?;
                }
            }
        }

        Item::None => (),
    }
    Ok(())
}

struct CssBuf {
    buf: Vec<u8>,
    format: Format,
    indent: usize,
    separate: bool,
}

impl CssBuf {
    fn new(format: Format, indent: usize) -> CssBuf {
        CssBuf {
            buf: Vec::new(),
            format,
            indent,
            separate: false,
        }
    }

    fn write_rule(&mut self, rule: &Rule) -> std::io::Result<()> {
        if !rule.body.is_empty() {
            self.do_indent_no_nl();
            if self.format.is_compressed() {
                write!(self.buf, "{:#}{{", rule.selectors)?;
            } else {
                write!(self.buf, "{} {{", rule.selectors)?;
            }

            self.indent += 2;
            for item in &rule.body {
                self.write_body_item(item)?;
            }
            self.indent -= 2;
            if self.format.is_compressed() && self.buf.last() == Some(&b';') {
                self.buf.pop();
            }
            self.do_indent();
            self.buf.write_all(if !self.format.is_compressed() {
                b"}\n"
            } else {
                b"}"
            })?;
        }
        Ok(())
    }

    fn write_body_item(&mut self, item: &BodyItem) -> std::io::Result<()> {
        self.do_indent();
        match item {
            BodyItem::Property(ref name, ref val) => write!(
                self.buf,
                "{}:{}{};",
                name,
                if self.format.is_compressed() { "" } else { " " },
                val.format(self.format).to_string().replace('\n', " "),
            ),
            BodyItem::Comment(ref c) => {
                let indent = self.indent;
                let existing = c
                    .lines()
                    .skip(1)
                    .map(|s| s.bytes().take_while(|b| *b == b' ').count())
                    .min()
                    .unwrap_or(0);
                let c = if existing < indent {
                    c.replace("\n", self.format.get_indent(indent - existing))
                } else {
                    c.clone()
                };
                self.buf.extend(b"/*");
                self.buf.extend(c.as_bytes());
                self.buf.extend(b"*/");
                Ok(())
            }
        }
    }

    pub fn add_import(
        &mut self,
        name: SassString,
        args: crate::css::Value,
    ) -> Result<(), Error> {
        self.do_indent_no_nl();
        write!(&mut self.buf, "@import {}", name)?;
        if !args.is_null() {
            write!(&mut self.buf, " {}", args.format(self.format))?;
        }
        self.buf.extend(if self.format.is_compressed() {
            &b";"[..]
        } else {
            &b";\n"[..]
        });
        Ok(())
    }

    fn do_separate(&mut self) {
        if self.separate {
            if !self.format.is_compressed() && !self.buf.is_empty() {
                self.buf.push(b'\n');
            }
        } else {
            self.separate = true;
        }
    }
    fn do_indent(&mut self) {
        self.buf
            .extend(self.format.get_indent(self.indent).as_bytes())
    }
    fn do_indent_no_nl(&mut self) {
        let stuff = self.format.get_indent(self.indent);
        if stuff.len() > 1 {
            self.buf.extend(stuff[1..].as_bytes())
        }
    }

    fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    fn is_ascii(&self) -> bool {
        self.buf.is_ascii()
    }

    fn join(&mut self, sub: Self) {
        self.buf.extend(sub.buf);
    }
}
