use css::Value;
use error::Error;
use file_context::FileContext;
use parser::parse_scss_file;
use sass::{FormalArgs, Item};
use selectors::{Selector, SelectorPart, Selectors};
use std::ascii::AsciiExt;
use std::fmt;
use std::io::Write;
use variablescope::{Scope, ScopeImpl};

/// Selected target format.
/// Only formats that are variants of this type are supported by rsass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputStyle {
    Normal, // TODO What should be the name of this format?
    Compressed,
}

impl OutputStyle {
    /// Write a slice of sass items in this format.
    /// The `file_context` is needed if there are `@import` statements
    /// in the sass file.
    pub fn write_root(&self,
                      items: &[Item],
                      globals: &mut Scope,
                      file_context: &FileContext)
                      -> Result<Vec<u8>, Error> {
        let mut result = CssWriter::new(*self);
        for item in items {
            self.handle_root_item(item, globals, file_context, &mut result)?;
        }
        result.get_result()
    }
    fn handle_root_item(&self,
                        item: &Item,
                        scope: &mut Scope,
                        file_context: &FileContext,
                        result: &mut CssWriter)
                        -> Result<(), Error> {
        match *item {
            Item::Import(ref name) => {
                let name = name.evaluate(scope).unquote();
                if let Value::Literal(ref x, _) = name {
                    if let Some((sub_context, file)) =
                        file_context.find_file(x.as_ref()) {
                        for item in parse_scss_file(&file)? {
                            self.handle_root_item(&item,
                                                  scope,
                                                  &sub_context,
                                                  result)?;
                        }
                    } else {
                        write!(result.to_imports(),
                               "@import url({});{}",
                               x,
                               if self.is_compressed() { "" } else { "\n" })?;
                    }
                } else {
                    write!(result.to_imports(),
                           "@import {};{}",
                           name,
                           if self.is_compressed() { "" } else { "\n" })?;
                }
            }
            Item::VariableDeclaration {
                ref name,
                ref val,
                ref default,
                ref global,
            } => {
                let val = val.evaluate(scope);
                if *default {
                    scope.define_default(name, &val, *global);
                } else if *global {
                    scope.define_global(name, &val);
                } else {
                    scope.define(name, &val);
                }
            }
            Item::AtRule { ref name, ref args, ref body } => {
                result.do_separate()?;
                let args = args.evaluate(scope);
                write!(result.to_content(), "@{} {}", name, args)?;
                if let Some(ref body) = *body {
                    if self.is_compressed() || args.is_null() {
                        write!(result.to_content(), "{{")?;
                    } else {
                        write!(result.to_content(), " {{")?;
                    }
                    let mut direct = vec![];
                    let mut sub = vec![];
                    self.handle_body(&mut direct,
                                     &mut sub,
                                     &mut ScopeImpl::sub(scope),
                                     &Selectors::root(),
                                     body,
                                     file_context,
                                     2)?;
                    if !sub.is_empty() {
                        result.do_indent(0)?;
                        result.to_content().write_all(&sub)?;
                    }
                    self.write_items(result.to_content(), &direct, 2)?;
                    write!(result.to_content(), "}}")?;
                } else {
                    write!(result.to_content(), ";")?;
                }
            }

            Item::MixinDeclaration { ref name, ref args, ref body } => {
                scope.define_mixin(name, args, body)
            }
            Item::MixinCall { ref name, ref args, ref body } => {
                if let Some((m_args, m_body)) = scope.get_mixin(name) {
                    let mut scope =
                        m_args.eval(scope, &args.evaluate(scope, true));
                    scope.define_mixin("%%BODY%%",
                                       &FormalArgs::default(),
                                       body);
                    for item in m_body {
                        self.handle_root_item(&item,
                                              &mut scope,
                                              file_context,
                                              result)?;
                    }
                } else {
                    panic!(format!("Unknown mixin {}({:?})", name, args))
                }
            }
            Item::Content => {
                panic!("@content not allowed in global context");
            }

            Item::FunctionDeclaration { ref name, ref func } => {
                scope.define_function(name, func.clone());
            }
            Item::Return(_) => {
                panic!("Return not allowed in global context");
            }

            Item::IfStatement(ref cond, ref do_if, ref do_else) => {
                let cond = cond.evaluate(scope).is_true();
                let items = if cond { do_if } else { do_else };
                for item in items {
                    self.handle_root_item(item, scope, file_context, result)?;
                }
            }
            Item::Each(ref name, ref values, ref body) => {
                for value in values.evaluate(scope).iter_items() {
                    scope.define(name, &value);
                    for item in body {
                        self.handle_root_item(item,
                                              scope,
                                              file_context,
                                              result)?;
                    }
                }
            }
            Item::For { ref name, ref from, ref to, inclusive, ref body } => {
                let from = from.evaluate(scope).integer_value()?;
                let to = to.evaluate(scope).integer_value()?;
                let to = if inclusive { to + 1 } else { to };
                for value in from..to {
                    let mut scope = ScopeImpl::sub(scope);
                    scope.define(name, &Value::scalar(value));
                    for item in body {
                        self.handle_root_item(item,
                                              &mut scope,
                                              file_context,
                                              result)?;
                    }
                }
            }
            Item::While(ref cond, ref body) => {
                let mut scope = ScopeImpl::sub(scope);
                while cond.evaluate(&scope).is_true() {
                    for item in body {
                        self.handle_root_item(item,
                                              &mut scope,
                                              file_context,
                                              result)?;
                    }
                }
            }

            Item::Rule(ref s, ref b) => {
                result.do_separate()?;
                self.write_rule(s,
                                b,
                                result.to_content(),
                                scope,
                                None,
                                file_context,
                                0)?;
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
    fn write_rule(&self,
                  selectors: &Selectors,
                  body: &[Item],
                  out: &mut Write,
                  scope: &mut Scope,
                  parent: Option<&Selectors>,
                  file_context: &FileContext,
                  indent: usize)
                  -> Result<(), Error> {
        let selectors = eval_selectors(selectors, scope).inside(parent);
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

    fn handle_body(&self,
                   direct: &mut Vec<CssBodyItem>,
                   sub: &mut Write,
                   scope: &mut Scope,
                   selectors: &Selectors,
                   body: &[Item],
                   file_context: &FileContext,
                   indent: usize)
                   -> Result<(), Error> {
        for b in body {
            match *b {
                Item::Import(ref name) => {
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
                Item::VariableDeclaration {
                    ref name,
                    ref val,
                    default,
                    global,
                } => {
                    let val = val.evaluate(scope);
                    if default {
                        scope.define_default(name, &val, global);
                    } else if global {
                        scope.define_global(name, &val);
                    } else {
                        scope.define(name, &val);
                    }
                }
                Item::AtRule { ref name, ref args, ref body } => {
                    write!(sub, "@{} {}", name, args.evaluate(scope))?;
                    if let Some(ref body) = *body {
                        if self.is_compressed() {
                            write!(sub, "{{")?;
                        } else {
                            write!(sub, " {{")?;
                        }

                        let mut s1 = vec![];
                        let mut s2 = vec![];
                        self.handle_body(&mut s1,
                                         &mut s2,
                                         &mut ScopeImpl::sub(scope),
                                         selectors,
                                         body,
                                         file_context,
                                         2)?;

                        if !s1.is_empty() {
                            self.do_indent(sub, 2)?;
                            if self.is_compressed() {
                                write!(sub, "{:#}{{", selectors)?;
                            } else {
                                write!(sub, "{} {{", selectors)?;
                            }
                            self.write_items(sub, &s1, 4)?;
                            write!(sub, "}}")?;
                        }
                        self.do_indent(sub, 0)?;
                        if !s2.is_empty() {
                            sub.write_all(&s2)?;
                        }
                        write!(sub, "}}")?;
                    } else {
                        write!(sub, ";")?;
                    }
                }

                Item::MixinDeclaration { ref name, ref args, ref body } => {
                    scope.define_mixin(name, args, body);
                }
                Item::MixinCall { ref name, ref args, ref body } => {
                    if let Some((m_args, m_body)) = scope.get_mixin(name) {
                        let mut argscope =
                            m_args.eval(scope, &args.evaluate(scope, true));
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
                Item::Content => {
                    if let Some((_args, m_body)) = scope.get_mixin("%%BODY%%") {
                        self.handle_body(direct,
                                         sub,
                                         scope,
                                         selectors,
                                         &m_body,
                                         file_context,
                                         indent)?;
                    }
                }

                Item::FunctionDeclaration { ref name, ref func } => {
                    scope.define_function(name, func.clone());
                }
                Item::Return(_) => {
                    panic!("Return not allowed in plain context");
                }

                Item::IfStatement(ref cond, ref do_if, ref do_else) => {
                    let cond = cond.evaluate(scope).is_true();
                    let items = if cond { do_if } else { do_else };
                    self.handle_body(direct,
                                     sub,
                                     &mut ScopeImpl::sub(scope),
                                     selectors,
                                     items,
                                     file_context,
                                     0)?;
                }
                Item::Each(ref name, ref values, ref body) => {
                    for value in values.evaluate(scope).iter_items() {
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
                Item::For {
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
                Item::While(ref cond, ref body) => {
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

                Item::Rule(ref s, ref b) => {
                    self.write_rule(s,
                                    b,
                                    sub,
                                    scope,
                                    Some(selectors),
                                    file_context,
                                    indent)?;
                }
                Item::NamespaceRule(ref name, ref value, ref body) => {
                    let value = value.evaluate(scope);
                    if !value.is_null() {
                        direct.push(CssBodyItem::Property(name.clone(),
                                                          value,
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
                Item::Property(ref name, ref value, ref important) => {
                    let v = value.evaluate(scope);
                    if !v.is_null() {
                        let (name, _q) = name.evaluate(scope);
                        direct.push(CssBodyItem::Property(name, v, *important));
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

    fn is_compressed(&self) -> bool {
        self == &OutputStyle::Compressed
    }
}

fn eval_selectors(s: &Selectors, scope: &Scope) -> Selectors {
    let s = Selectors(s.0
                          .iter()
                          .map(|s| {
        Selector(s.0
                     .iter()
                     .map(|sp| match *sp {
                              SelectorPart::Attribute {
                                  ref name,
                                  ref op,
                                  ref val,
                              } => {
                                  SelectorPart::Attribute {
                                      name: name.evaluate2(scope),
                                      op: op.clone(),
                                      val: val.evaluate2(scope),
                                  }
                              }
                              SelectorPart::Simple(ref v) => {
                                  SelectorPart::Simple(v.evaluate2(scope))
                              }
                              SelectorPart::Pseudo { ref name, ref arg } => {
                                  SelectorPart::Pseudo {
                                      name: name.evaluate2(scope),
                                      arg: arg.as_ref()
                                          .map(|a| {
                                                   eval_selectors(a, scope)
                                               }),
                                  }
                              }
                              SelectorPart::PseudoElement(ref e) => {
                                  let e = e.evaluate2(scope);
                                  SelectorPart::PseudoElement(e)
                              }
                              ref sp => sp.clone(),
                          })
                     .collect())
    })
                          .collect());
    // The "simple" parts we get from evaluating interpolations may
    // contain high-level selector separators (i.e. ","), so we need to
    // parse the selectors again, from a string representation.
    use parser::selectors::selectors;
    selectors(format!("{} ", s).as_bytes()).unwrap().1
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
            style: style,
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
            write!(&mut result, "\n")?;
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
                write!(self.contents, "\n")?;
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
