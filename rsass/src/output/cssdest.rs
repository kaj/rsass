use super::CssData;
use crate::css::{
    AtRule, AtRuleBodyItem, Comment, CssSelectorSet, CssString,
    CustomProperty, Import, Item, MediaArgs, MediaRule, Property, Rule,
    Value,
};
use crate::Invalid;

type Result<T = ()> = std::result::Result<T, Invalid>;

pub trait CssDestination {
    fn head(&mut self) -> &mut CssData;

    fn start_rule(
        &mut self,
        selectors: CssSelectorSet,
    ) -> Result<RuleDest<'_>>;
    fn start_atmedia(&mut self, args: MediaArgs) -> AtMediaDest<'_>;
    fn start_atrule(&mut self, name: String, args: Value) -> AtRuleDest<'_>;
    fn start_nsrule(&mut self, name: String) -> Result<NsRuleDest<'_>>;

    fn push_import(&mut self, import: Import);
    fn push_comment(&mut self, c: Comment);
    fn push_item(&mut self, item: Item) -> Result;
    fn push_property(&mut self, name: String, value: Value) -> Result;
    fn push_custom_property(
        &mut self,
        name: String,
        value: CssString,
    ) -> Result;

    /// Nop in default implementation, adds a spacer in `CssHead`.
    fn separate(&mut self) {}
}

pub struct RuleDest<'a> {
    parent: &'a mut dyn CssDestination,
    rule: Rule,
    trail: Vec<Item>,
}

impl<'a> RuleDest<'a> {
    pub fn new(
        parent: &'a mut dyn CssDestination,
        selectors: CssSelectorSet,
    ) -> Self {
        RuleDest {
            parent,
            rule: Rule::new(selectors.into()),
            trail: Default::default(),
        }
    }
}
impl Drop for RuleDest<'_> {
    fn drop(&mut self) {
        fn end(dest: &mut RuleDest) -> Result<()> {
            dest.parent.push_item(dest.rule.clone().into())?;
            for item in std::mem::take(&mut dest.trail) {
                dest.parent.push_item(item)?;
            }
            dest.parent.separate();
            Ok(())
        }
        if let Err(err) = end(self) {
            eprintln!("Error in ending RuleDest: {err}");
        }
    }
}

impl CssDestination for RuleDest<'_> {
    fn head(&mut self) -> &mut CssData {
        self.parent.head()
    }
    fn start_rule(
        &mut self,
        selectors: CssSelectorSet,
    ) -> Result<RuleDest<'_>> {
        Ok(RuleDest::new(self, selectors))
    }
    fn start_atmedia(&mut self, args: MediaArgs) -> AtMediaDest<'_> {
        let selectors = self.rule.selectors.clone();
        AtMediaDest {
            parent: self,
            args,
            rule: Some(Rule::new(selectors)),
            body: Vec::new(),
        }
    }
    fn start_atrule(&mut self, name: String, args: Value) -> AtRuleDest<'_> {
        let rule = if is_flat_rule(&name) {
            None
        } else {
            Some(Rule::new(self.rule.selectors.clone()))
        };
        AtRuleDest {
            parent: self,
            name,
            args,
            rule,
            body: Vec::new(),
        }
    }
    fn start_nsrule(&mut self, name: String) -> Result<NsRuleDest<'_>> {
        Ok(NsRuleDest { parent: self, name })
    }

    fn push_import(&mut self, import: Import) {
        self.rule.push(import.into());
    }

    fn push_comment(&mut self, c: Comment) {
        self.rule.push(c.into());
    }

    fn push_item(&mut self, item: Item) -> Result {
        match item {
            Item::AtRule(r) => match r.try_into() {
                Ok(item) => self.rule.push(item),
                Err(r) => self.trail.push(r.into()),
            },
            item => self.trail.push(item),
        }
        Ok(())
    }

    fn push_property(&mut self, name: String, value: Value) -> Result {
        self.rule.push(Property::new(name, value).into());
        Ok(())
    }

    fn push_custom_property(
        &mut self,
        name: String,
        value: CssString,
    ) -> Result {
        self.rule.push(CustomProperty::new(name, value).into());
        Ok(())
    }
}

pub struct NsRuleDest<'a> {
    parent: &'a mut dyn CssDestination,
    name: String,
}

impl CssDestination for NsRuleDest<'_> {
    fn head(&mut self) -> &mut CssData {
        self.parent.head()
    }
    fn start_rule(
        &mut self,
        _selectors: CssSelectorSet,
    ) -> Result<RuleDest<'_>> {
        Err(Invalid::InNsRule)
    }
    fn start_atmedia(&mut self, args: MediaArgs) -> AtMediaDest<'_> {
        AtMediaDest::new(self, args)
    }
    fn start_atrule(&mut self, name: String, args: Value) -> AtRuleDest<'_> {
        AtRuleDest {
            parent: self,
            name,
            args,
            rule: None,
            body: Vec::new(),
        }
    }
    fn start_nsrule(&mut self, name: String) -> Result<NsRuleDest<'_>> {
        Ok(NsRuleDest { parent: self, name })
    }

    fn push_import(&mut self, import: Import) {
        self.parent.push_import(import);
    }
    fn push_comment(&mut self, c: Comment) {
        self.parent.push_comment(c);
    }
    fn push_item(&mut self, _item: Item) -> Result {
        Err(Invalid::InNsRule)
    }
    fn push_property(&mut self, name: String, value: Value) -> Result {
        self.parent
            .push_property(format!("{}-{}", self.name, name), value)
    }
    fn push_custom_property(&mut self, _: String, _: CssString) -> Result {
        Err(Invalid::InNsRule)
    }
}

pub struct AtRuleDest<'a> {
    parent: &'a mut dyn CssDestination,
    name: String,
    args: Value,
    rule: Option<Rule>,
    body: Vec<AtRuleBodyItem>,
}
impl<'a> AtRuleDest<'a> {
    pub fn new(
        parent: &'a mut dyn CssDestination,
        name: String,
        args: Value,
    ) -> Self {
        AtRuleDest {
            parent,
            name,
            args,
            rule: None,
            body: Vec::new(),
        }
    }
}

impl Drop for AtRuleDest<'_> {
    fn drop(&mut self) {
        let mut body = std::mem::take(&mut self.body);
        let name = std::mem::take(&mut self.name);
        let args = std::mem::replace(&mut self.args, Value::Null);
        if let Some(rule) = self.rule.take() {
            body.insert(0, rule.into());
        }
        let result = AtRule::new(name, args, Some(body));
        if let Err(err) = self.parent.push_item(result.into()) {
            eprintln!("Error ending AtRuleDest: {err}");
        }
        self.parent.separate();
    }
}
impl CssDestination for AtRuleDest<'_> {
    fn head(&mut self) -> &mut CssData {
        self.parent.head()
    }
    fn start_rule(
        &mut self,
        selectors: CssSelectorSet,
    ) -> Result<RuleDest<'_>> {
        Ok(RuleDest::new(self, selectors))
    }
    fn start_atmedia(&mut self, args: MediaArgs) -> AtMediaDest<'_> {
        let rule = self.rule.as_ref().map(|r| Rule::new(r.selectors.clone()));
        AtMediaDest {
            parent: self,
            args,
            rule,
            body: Vec::new(),
        }
    }
    fn start_atrule(&mut self, name: String, args: Value) -> AtRuleDest<'_> {
        let rule = if is_flat_rule(&name) {
            None
        } else {
            self.rule.as_ref().map(|r| Rule::new(r.selectors.clone()))
        };
        AtRuleDest {
            parent: self,
            name,
            args,
            rule,
            body: Vec::new(),
        }
    }
    fn start_nsrule(&mut self, name: String) -> Result<NsRuleDest<'_>> {
        Ok(NsRuleDest { parent: self, name })
    }

    fn push_import(&mut self, import: Import) {
        self.body.push(import.into());
    }

    fn push_comment(&mut self, c: Comment) {
        if let Some(rule) = &mut self.rule {
            rule.push(c.into());
        } else {
            self.body.push(c.into());
        }
    }

    fn push_item(&mut self, item: Item) -> Result {
        self.body.push(match item {
            Item::Comment(c) => c.into(),
            Item::Import(i) => i.into(),
            Item::Rule(r) => r.into(),
            // FIXME: This should bubble or something?
            Item::MediaRule(r) => r.into(),
            Item::AtRule(r) => r.into(),
            Item::Separator => return Ok(()), // Not pushed?
        });
        Ok(())
    }

    fn push_property(&mut self, name: String, value: Value) -> Result {
        let prop = Property::new(name, value);
        if let Some(rule) = &mut self.rule {
            rule.push(prop.into());
        } else {
            self.body.push(prop.into());
        }
        Ok(())
    }

    fn push_custom_property(
        &mut self,
        name: String,
        value: CssString,
    ) -> Result {
        if let Some(rule) = &mut self.rule {
            rule.push(CustomProperty::new(name, value).into());
            Ok(())
        } else {
            Err(Invalid::GlobalCustomProperty)
        }
    }
}

pub struct AtMediaDest<'a> {
    parent: &'a mut dyn CssDestination,
    args: MediaArgs,
    rule: Option<Rule>,
    body: Vec<AtRuleBodyItem>,
}
impl<'a> AtMediaDest<'a> {
    pub fn new(parent: &'a mut dyn CssDestination, args: MediaArgs) -> Self {
        AtMediaDest {
            parent,
            args,
            rule: None,
            body: Vec::new(),
        }
    }
}

impl Drop for AtMediaDest<'_> {
    fn drop(&mut self) {
        let mut body = std::mem::take(&mut self.body);
        let args =
            std::mem::replace(&mut self.args, MediaArgs::Name(String::new()));
        if let Some(rule) = self.rule.take() {
            if !rule.body.is_empty() {
                body.insert(0, rule.into());
            }
        }
        let result = MediaRule::new(args, body);
        if let Err(err) = self.parent.push_item(result.into()) {
            eprintln!("Error ending AtRuleDest: {err}");
        }
        self.parent.separate();
    }
}

impl CssDestination for AtMediaDest<'_> {
    fn head(&mut self) -> &mut CssData {
        self.parent.head()
    }

    fn start_rule(
        &mut self,
        selectors: CssSelectorSet,
    ) -> Result<RuleDest<'_>> {
        Ok(RuleDest::new(self, selectors))
    }
    fn start_atmedia(&mut self, args: MediaArgs) -> AtMediaDest<'_> {
        let rule = self.rule.as_ref().map(|r| Rule::new(r.selectors.clone()));
        AtMediaDest {
            parent: self,
            args,
            rule,
            body: Vec::new(),
        }
    }
    fn start_atrule(&mut self, name: String, args: Value) -> AtRuleDest<'_> {
        let rule = if is_flat_rule(&name) {
            None
        } else {
            self.rule.as_ref().map(|r| Rule::new(r.selectors.clone()))
        };
        AtRuleDest {
            parent: self,
            name,
            args,
            rule,
            body: Vec::new(),
        }
    }
    fn start_nsrule(&mut self, name: String) -> Result<NsRuleDest<'_>> {
        Ok(NsRuleDest { parent: self, name })
    }

    fn push_import(&mut self, import: Import) {
        self.body.push(import.into());
    }

    fn push_comment(&mut self, c: Comment) {
        if let Some(rule) = &mut self.rule {
            rule.push(c.into());
        } else {
            self.body.push(c.into());
        }
    }

    fn push_item(&mut self, item: Item) -> Result {
        self.body.push(match item {
            Item::Comment(c) => c.into(),
            Item::Import(i) => i.into(),
            Item::Rule(r) => r.into(),
            // FIXME: Check if the args can be merged!
            // Or is that a separate pass after building a first css tree?
            Item::MediaRule(r) => r.into(),
            Item::AtRule(r) => r.into(),
            Item::Separator => return Ok(()), // Not pushed?
        });
        Ok(())
    }

    fn push_property(&mut self, name: String, value: Value) -> Result {
        let prop = Property::new(name, value);
        if let Some(rule) = &mut self.rule {
            rule.push(prop.into());
        } else {
            self.body.push(prop.into());
        }
        Ok(())
    }

    fn push_custom_property(
        &mut self,
        name: String,
        value: CssString,
    ) -> Result {
        if let Some(rule) = &mut self.rule {
            rule.push(CustomProperty::new(name, value).into());
            Ok(())
        } else {
            Err(Invalid::GlobalCustomProperty)
        }
    }
}

fn is_flat_rule(name: &str) -> bool {
    name == "font-face" || name == "keyframes"
}
