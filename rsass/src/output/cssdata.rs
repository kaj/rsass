use super::cssdest::{
    AtMediaDest, AtRuleDest, CssDestination, NsRuleDest, RuleDest,
};
use super::{CssBuf, Format};
use crate::css::{
    Comment, CssString, Import, Item, MediaArgs, Selectors, Value,
};
use crate::{Error, Invalid, ScopeRef};
use std::collections::BTreeMap;

type Result<T, E = Invalid> = std::result::Result<T, E>;

/// A holder for css data.
pub struct CssData {
    imports: Vec<Import>,
    body: Vec<Item>,
    modules: BTreeMap<String, ScopeRef>,
}

impl CssData {
    pub fn new() -> Self {
        CssData {
            imports: Default::default(),
            body: Default::default(),
            modules: Default::default(),
        }
    }
    pub fn into_iter(self) -> impl Iterator<Item = Item> {
        self.imports.into_iter().map(Into::into).chain(self.body)
    }
    pub fn load_module<Init>(
        &mut self,
        path: &str,
        init: Init,
    ) -> Result<ScopeRef, Error>
    where
        Init: FnOnce(&mut Self) -> Result<ScopeRef, Error>,
    {
        if let Some(loaded) = self.modules.get(path) {
            return Ok(loaded.clone());
        }
        let module = init(self)?;
        self.modules.insert(path.into(), module.clone());
        Ok(module)
    }

    pub fn into_buffer(self, format: Format) -> Result<Vec<u8>, Error> {
        let mut buf = CssBuf::new(format);
        for i in &self.imports {
            i.write(&mut buf)?;
        }
        for i in &self.body {
            i.write(&mut buf)?;
        }
        let buf = buf.take();
        let compressed = format.is_compressed();
        let mut result = if buf.is_ascii() {
            buf
        } else {
            let mark = if compressed {
                // U+FEFF is byte order mark, used to show encoding.
                "\u{feff}"
            } else {
                "@charset \"UTF-8\";\n"
            };
            let mut result = Vec::with_capacity(mark.len() + buf.len());
            result.extend_from_slice(mark.as_bytes());
            result.extend(buf);
            result
        };
        while result.last() == Some(&b'\n') {
            result.pop();
        }
        if compressed && result.last() == Some(&b';') {
            result.pop();
        }
        if !result.is_empty() {
            result.push(b'\n');
        }
        Ok(result)
    }
}

impl CssDestination for CssData {
    fn head(&mut self) -> &mut CssData {
        self
    }

    fn start_rule(&mut self, selectors: Selectors) -> Result<RuleDest> {
        Ok(RuleDest::new(self, selectors))
    }
    fn start_atmedia(&mut self, args: MediaArgs) -> AtMediaDest {
        AtMediaDest::new(self, args)
    }
    fn start_atrule(&mut self, name: String, args: Value) -> AtRuleDest {
        AtRuleDest::new(self, name, args)
    }
    fn start_nsrule(&mut self, _name: String) -> Result<NsRuleDest> {
        Err(Invalid::GlobalNsProperty)
    }

    fn push_import(&mut self, import: Import) {
        self.imports.push(import);
    }

    fn push_comment(&mut self, c: Comment) {
        self.body.push(c.into());
    }

    fn push_item(&mut self, item: Item) -> Result<()> {
        if let Item::Import(import) = item {
            self.push_import(import);
        } else {
            self.body.push(item);
        }
        Ok(())
    }

    fn push_property(&mut self, _name: String, _value: Value) -> Result<()> {
        Err(Invalid::DeclarationOutsideRule)
    }

    fn push_custom_property(
        &mut self,
        _: String,
        _: CssString,
    ) -> Result<()> {
        Err(Invalid::GlobalCustomProperty)
    }

    fn separate(&mut self) {
        self.body.push(Item::Separator);
    }
}
