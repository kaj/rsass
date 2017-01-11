use selectors::Selector;
use std::io::{self, Write};
use super::SassItem;
use variablescope::{ScopeImpl, Scope};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputStyle {
    Normal, // TODO What should be the name of this format?
    Compressed,
}

impl OutputStyle {
    pub fn write_rule(&self,
                      selectors: &[Selector],
                      body: &[SassItem],
                      out: &mut Write,
                      scope: &mut Scope,
                      parent: Option<&[Selector]>,
                      indent: usize)
                      -> io::Result<()> {
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
        try!(self.handle_body(&mut direct,
                              &mut sub,
                              &mut ScopeImpl::sub(scope),
                              &selectors,
                              &body,
                              indent));
        if !direct.is_empty() {
            try!(write!(out,
                        "{}{}{{",
                        selectors.iter()
                            .map(|s| format!("{}", s))
                            .collect::<Vec<_>>()
                            .join(", "),
                        self.opt_space()));
            if self.is_compressed() && direct.last() == Some(&b';') {
                direct.pop();
            }
            try!(out.write(&direct));
            try!(self.do_indent(out, indent));
            try!(write!(out, "}}"));
            try!(self.do_indent(out, 0));
        }
        try!(out.write(&sub));
        Ok(())
    }

    pub fn handle_body(&self,
                       direct: &mut Vec<u8>,
                       sub: &mut Vec<u8>,
                       scope: &mut Scope,
                       selectors: &[Selector],
                       body: &[SassItem],
                       indent: usize)
                       -> io::Result<()> {
        for b in body {
            match b {
                &SassItem::Comment(ref c) => {
                    try!(self.do_indent(direct, indent + 2));
                    try!(write!(direct, "/*{}*/", c));
                }
                &SassItem::Property(ref name, ref value) => {
                    try!(self.do_indent(direct, indent + 2));
                    try!(write!(direct,
                                "{}:{}{};",
                                name,
                                self.opt_space(),
                                scope.evaluate(value)));
                }
                &SassItem::Rule(ref s, ref b) => {
                    try!(self.write_rule(s,
                                         b,
                                         sub,
                                         scope,
                                         Some(&selectors),
                                         indent));
                }
                &SassItem::VariableDeclaration { ref name,
                                                 ref val,
                                                 global } => {
                    scope.define(&name, &val, global);
                }
                &SassItem::MixinDeclaration(ref m) => {
                    scope.define_mixin(m);
                }
                &SassItem::MixinCall { ref name, ref args } => {
                    if let Some(mixin) = scope.get_mixin(name) {
                        let mut argscope = mixin.argscope(scope, &args);
                        try!(self.handle_body(direct,
                                              sub,
                                              &mut argscope,
                                              selectors,
                                              &mixin.body,
                                              indent));
                    } else {
                        writeln!(direct,
                                 "/* Unknown mixin {}({:?}) */",
                                 name,
                                 args)
                            .unwrap();
                    }
                }
                &SassItem::None => (),
            }
        }
        Ok(())
    }

    pub fn do_indent(&self, out: &mut Write, steps: usize) -> io::Result<()> {
        if !self.is_compressed() {
            try!(write!(out, "\n"));
            for _i in 0..steps {
                try!(write!(out, " "));
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
