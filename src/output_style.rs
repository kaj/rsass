use super::{FileContext, SassItem, parse_scss_file};
use error::Error;
use formalargs::FormalArgs;
use selectors::Selector;
use std::ascii::AsciiExt;
use std::fmt;
use std::io::Write;
use valueexpression::Value;
use variablescope::{GlobalScope, Scope, ScopeImpl};

/// Selected target format.
/// Only formats that are variants of this type are supported by rsass.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputStyle {
    Normal, // TODO What should be the name of this format?
    Compressed,
}

impl OutputStyle {
    /// Write a slice of sass items in this format.
    /// The `file_context` is needed if there are `@import` statements
    /// in the sass file.
    pub fn write_root(&self,
                      items: &[SassItem],
                      file_context: FileContext)
                      -> Result<Vec<u8>, Error> {
        let mut globals = GlobalScope::new();
        let mut result = Vec::new();
        let mut separate = false;
        for item in items {
            self.handle_root_item(item,
                                  &mut globals,
                                  &mut separate,
                                  &file_context,
                                  &mut result)?;
        }
        if result != b"" && result[result.len() - 1] != b'\n' {
            write!(result, "\n")?;
        }
        if result.is_ascii() {
            Ok(result)
        } else {
            let mut r2 = vec![];
            if self.is_compressed() {
                // Byte order mark U+FEFF as utf-8.
                r2.extend_from_slice("\u{feff}".as_bytes());
            } else {
                r2.extend_from_slice(b"@charset \"UTF-8\";\n");
            }
            r2.extend(result);
            Ok(r2)
        }
    }
    fn handle_root_item(&self,
                        item: &SassItem,
                        globals: &mut Scope,
                        separate: &mut bool,
                        file_context: &FileContext,
                        result: &mut Write)
                        -> Result<(), Error> {
        match *item {
            SassItem::Rule(ref s, ref b) => {
                if *separate {
                    self.do_indent(result, 0)?;
                } else {
                    *separate = true;
                }
                self.write_rule(s, b, result, globals, None, file_context, 0)?;
            }
            SassItem::VariableDeclaration {
                ref name,
                ref val,
                ref default,
                ref global,
            } => {
                if *default {
                    globals.define_default(name, val, *global);
                } else if *global {
                    globals.define_global(name, val);
                } else {
                    globals.define(name, val);
                }
            }
            SassItem::MixinDeclaration { ref name, ref args, ref body } => {
                globals.define_mixin(name, args, body)
            }
            SassItem::FunctionDeclaration { ref name, ref func } => {
                globals.define_function(name, func.clone());
            }
            SassItem::MixinCall { ref name, ref args, ref body } => {
                if let Some((m_args, m_body)) = globals.get_mixin(name) {
                    let mut scope = m_args.eval(globals, args);
                    scope.define_mixin("%%BODY%%",
                                       &FormalArgs::default(),
                                       body);
                    for item in m_body {
                        self.handle_root_item(&item,
                                              &mut scope,
                                              separate,
                                              file_context,
                                              result)?;
                    }
                } else {
                    panic!(format!("Unknown mixin {}({:?})", name, args))
                }
            }
            SassItem::Import(ref name) => {
                let name = name.evaluate(globals);
                if let Value::Literal(ref x, _) = name {
                    if let Some((sub_context, file)) =
                        file_context.find_file(x.as_ref()) {
                        for item in parse_scss_file(&file)? {
                            self.handle_root_item(&item,
                                                  globals,
                                                  separate,
                                                  &sub_context,
                                                  result)?;
                        }
                    } else {
                        writeln!(result, "@import url({});", x)?;
                    }
                } else {
                    writeln!(result, "@import {};", name)?;
                }
            }
            SassItem::Comment(ref c) => {
                if !self.is_compressed() {
                    if *separate {
                        self.do_indent(result, 0)?;
                    } else {
                        *separate = true;
                    }
                    write!(result, "/*{}*/", c)?;
                }
            }
            SassItem::Property(..) => {
                panic!("Global property not allowed");
            }
            SassItem::NamespaceRule(..) => {
                panic!("Global namespaced property not allowed");
            }
            SassItem::AtRule(ref query, ref body) => {
                if *separate {
                    self.do_indent(result, 0)?;
                } else {
                    *separate = true;
                }
                write!(result, "@{}{{", query)?;
                let mut direct = vec![];
                let mut sub = vec![];
                self.handle_body(&mut direct,
                                 &mut sub,
                                 &mut ScopeImpl::sub(globals),
                                 &[Selector::root()],
                                 body,
                                 file_context,
                                 2)?;
                if !sub.is_empty() {
                    self.do_indent(result, 0)?;
                    result.write_all(&sub)?;
                }
                self.write_items(result, &direct, 2)?;
                write!(result, "}}")?;
            }
            SassItem::IfStatement(ref cond, ref do_if, ref do_else) => {
                if cond.evaluate(globals).is_true() {
                    for item in do_if {
                        self.handle_root_item(item,
                                              globals,
                                              separate,
                                              file_context,
                                              result)?;
                    }
                } else {
                    for item in do_else {
                        self.handle_root_item(item,
                                              globals,
                                              separate,
                                              file_context,
                                              result)?;
                    }
                }
            }
            SassItem::Each(ref name, ref values, ref body) => {
                let values = match values.evaluate(globals) {
                    Value::List(v, _) => v,
                    v => vec![v],
                };
                for value in values {
                    globals.define(name, &value);
                    for item in body {
                        self.handle_root_item(item,
                                              globals,
                                              separate,
                                              file_context,
                                              result)?;
                    }
                }
            }
            SassItem::For {
                ref name,
                ref from,
                ref to,
                inclusive,
                ref body,
            } => {
                let from = from.evaluate(globals).integer_value()?;
                let to = to.evaluate(globals).integer_value()?;
                let to = if inclusive { to + 1 } else { to };
                for value in from..to {
                    let mut scope = ScopeImpl::sub(globals);
                    scope.define(name, &Value::scalar(value));
                    for item in body {
                        self.handle_root_item(item,
                                              &mut scope,
                                              separate,
                                              file_context,
                                              result)?;
                    }
                }
            }
            SassItem::While(ref cond, ref body) => {
                let mut scope = ScopeImpl::sub(globals);
                while cond.evaluate(&scope).is_true() {
                    for item in body {
                        self.handle_root_item(item,
                                              &mut scope,
                                              separate,
                                              file_context,
                                              result)?;
                    }
                }
            }
            SassItem::Return(_) => {
                panic!("Return not allowed in global context");
            }
            SassItem::Content => {
                panic!("@content not allowed in global context");
            }
            SassItem::None => (),
        }
        Ok(())
    }
    fn write_rule(&self,
                  selectors: &[Selector],
                  body: &[SassItem],
                  out: &mut Write,
                  scope: &mut Scope,
                  parent: Option<&[Selector]>,
                  file_context: &FileContext,
                  indent: usize)
                  -> Result<(), Error> {
        let selectors = if let Some(parent) = parent {
            let mut result = Vec::new();
            for p in parent {
                for s in selectors {
                    result.push(p.join(s));
                }
            }
            result
        } else {
            selectors.into()
        };
        let mut direct = Vec::new();
        let mut sub = Vec::new();
        self.handle_body(&mut direct,
                         &mut sub,
                         &mut ScopeImpl::sub(scope),
                         &selectors,
                         body,
                         file_context,
                         indent)?;
        if !direct.is_empty() {
            self.do_indent_no_lf(out, indent)?;
            write!(out,
                   "{}{}{{",
                   self.join_selectors(&selectors),
                   self.opt_space())?;
            self.write_items(out, &direct, indent + 2)?;
            write!(out, "}}")?;
            self.do_indent(out, 0)?;
        }
        out.write_all(&sub)?;
        Ok(())
    }

    fn join_selectors(&self, selectors: &[Selector]) -> String {
        selectors
            .iter()
            .map(|s| if self.is_compressed() {
                     format!("{:#}", s)
                 } else {
                     format!("{}", s)
                 })
            .collect::<Vec<_>>()
            .join(if self.is_compressed() { "," } else { ", " })
    }

    fn handle_body(&self,
                   direct: &mut Vec<CssBodyItem>,
                   sub: &mut Write,
                   scope: &mut Scope,
                   selectors: &[Selector],
                   body: &[SassItem],
                   file_context: &FileContext,
                   indent: usize)
                   -> Result<(), Error> {
        for b in body {
            match *b {
                SassItem::Comment(ref c) => {
                    if !self.is_compressed() {
                        direct.push(CssBodyItem::Comment(c.clone()));
                    }
                }
                SassItem::Property(ref name, ref value, ref important) => {
                    let v = value.evaluate(scope);
                    if !v.is_null() {
                        direct.push(CssBodyItem::Property(name.clone(),
                                                          v,
                                                          *important));
                    }
                }
                SassItem::NamespaceRule(ref name, ref value, ref body) => {
                    if !value.is_null() {
                        direct.push(CssBodyItem::Property(name.clone(),
                                                          value.evaluate(scope),
                                                          false));
                    }
                    let mut t = Vec::new();
                    self.handle_body(&mut t,
                                     sub,
                                     scope,
                                     selectors,
                                     body,
                                     file_context,
                                     indent)?;
                    for item in t {
                        direct.push(match item {
                                        CssBodyItem::Property(n, v, i) => {
                                            CssBodyItem::Property(
                                                format!("{}-{}", name, n),
                                                v,
                                                i)
                                        }
                                        c => c,
                                    })
                    }
                }
                SassItem::Rule(ref s, ref b) => {
                    self.write_rule(s,
                                    b,
                                    sub,
                                    scope,
                                    Some(selectors),
                                    file_context,
                                    indent)?;
                }
                SassItem::VariableDeclaration {
                    ref name,
                    ref val,
                    default,
                    global,
                } => {
                    if default {
                        scope.define_default(name, val, global);
                    } else if global {
                        scope.define_global(name, val);
                    } else {
                        scope.define(name, val);
                    }
                }
                SassItem::MixinDeclaration { ref name, ref args, ref body } => {
                    scope.define_mixin(name, args, body);
                }
                SassItem::FunctionDeclaration { ref name, ref func } => {
                    scope.define_function(name, func.clone());
                }
                SassItem::MixinCall { ref name, ref args, ref body } => {
                    if let Some((m_args, m_body)) = scope.get_mixin(name) {
                        let mut argscope = m_args.eval(scope, args);
                        argscope.define_mixin("%%BODY%%",
                                              &FormalArgs::default(),
                                              body);
                        self.handle_body(direct,
                                         sub,
                                         &mut argscope,
                                         selectors,
                                         &m_body,
                                         file_context,
                                         indent)?;
                    } else {
                        direct.push(CssBodyItem::Comment(
                            format!("Unknown mixin {}({:?})", name, args)));
                    }
                }
                SassItem::Content => {
                    if let Some((_args, m_body)) = scope.get_mixin("%%BODY%%") {
                        self.handle_body(direct,
                                         sub,
                                         scope,
                                         selectors,
                                         &m_body,
                                         file_context,
                                         indent)?;
                    } else {
                        direct.push(CssBodyItem::Comment(
                            "Mixin @content not found.".to_string()));
                    }
                }
                SassItem::Import(ref name) => {
                    let name = name.evaluate(scope);
                    if let Value::Literal(ref x, _) = name {
                        let (sub_context, file) = file_context.file(x.as_ref());
                        let items = parse_scss_file(&file)?;
                        self.handle_body(direct,
                                         sub,
                                         scope,
                                         selectors,
                                         &items,
                                         &sub_context,
                                         0)?;
                    } else {
                        // TODO writeln!(direct, "@import {};", name)?;
                    }
                }
                SassItem::AtRule(ref query, ref body) => {
                    let mut s1 = vec![];
                    let mut s2 = vec![];
                    self.handle_body(&mut s1,
                                     &mut s2,
                                     &mut ScopeImpl::sub(scope),
                                     selectors,
                                     body,
                                     file_context,
                                     2)?;

                    write!(sub, "@{}{{", query)?;
                    if !s1.is_empty() {
                        self.do_indent(sub, 2)?;
                        write!(sub,
                               "{}{}{{",
                               self.join_selectors(selectors),
                               self.opt_space())?;
                        self.write_items(sub, &s1, 4)?;
                        write!(sub, "}}")?;
                    }
                    if !s2.is_empty() {
                        self.do_indent(sub, 0)?;
                        sub.write_all(&s2)?;
                    }
                    write!(sub, "}}")?;
                }
                SassItem::IfStatement(ref cond, ref do_if, ref do_else) => {
                    if cond.evaluate(scope).is_true() {
                        self.handle_body(direct,
                                         sub,
                                         &mut ScopeImpl::sub(scope),
                                         selectors,
                                         do_if,
                                         file_context,
                                         0)?;
                    } else {
                        self.handle_body(direct,
                                         sub,
                                         &mut ScopeImpl::sub(scope),
                                         selectors,
                                         do_else,
                                         file_context,
                                         0)?;
                    }
                }
                SassItem::Each(ref name, ref values, ref body) => {
                    let values = match values.evaluate(scope) {
                        Value::List(v, _) => v,
                        v => vec![v],
                    };
                    for value in values {
                        let mut scope = ScopeImpl::sub(scope);
                        scope.define(name, &value);
                        self.handle_body(direct,
                                         sub,
                                         &mut scope,
                                         selectors,
                                         body,
                                         file_context,
                                         0)?;
                    }
                }
                SassItem::For {
                    ref name,
                    ref from,
                    ref to,
                    inclusive,
                    ref body,
                } => {
                    let from = from.evaluate(scope).integer_value()?;
                    let to = to.evaluate(scope).integer_value()?;
                    let to = if inclusive { to + 1 } else { to };
                    for value in from..to {
                        let mut scope = ScopeImpl::sub(scope);
                        scope.define(name, &Value::scalar(value));
                        self.handle_body(direct,
                                         sub,
                                         &mut scope,
                                         selectors,
                                         body,
                                         file_context,
                                         0)?;
                    }
                }
                SassItem::While(ref cond, ref body) => {
                    let mut scope = ScopeImpl::sub(scope);
                    while cond.evaluate(&scope).is_true() {
                        self.handle_body(direct,
                                         sub,
                                         &mut scope,
                                         selectors,
                                         body,
                                         file_context,
                                         0)?;
                    }
                }
                SassItem::Return(_) => {
                    panic!("Return not allowed in plain context");
                }
                SassItem::None => (),
            }
        }
        Ok(())
    }

    fn write_items(&self,
                   out: &mut Write,
                   items: &[CssBodyItem],
                   indent: usize)
                   -> Result<(), Error> {
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
            write!(out, "\n")?;
            for _i in 0..steps {
                write!(out, " ")?;
            }
        }
        Ok(())
    }
    fn do_indent_no_lf(&self,
                       out: &mut Write,
                       steps: usize)
                       -> Result<(), Error> {
        if !self.is_compressed() {
            for _i in 0..steps {
                write!(out, " ")?;
            }
        }
        Ok(())
    }

    fn opt_space(&self) -> &'static str {
        if self.is_compressed() { "" } else { " " }
    }

    fn is_compressed(&self) -> bool {
        self == &OutputStyle::Compressed
    }
}

enum CssBodyItem {
    Property(String, Value, bool),
    Comment(String),
}

impl fmt::Display for CssBodyItem {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CssBodyItem::Property(ref name, ref val, ref imp) => {
                if out.alternate() {
                    let important = if *imp { "!important" } else { "" };
                    write!(out, "{}:{:#}{};", name, val, important)
                } else {
                    let important = if *imp { " !important" } else { "" };
                    write!(out, "{}: {}{};", name, val, important)
                }
            }
            CssBodyItem::Comment(ref c) => write!(out, "/*{}*/", c),
        }
    }
}
