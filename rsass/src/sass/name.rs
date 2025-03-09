use std::borrow::Cow;
use std::fmt;

/// A sass name, used to idenify functions, variables, mixins, etc.
///
/// A `-` and a `_` is considered equal in a name, both represented by a `_`.
///
/// # Examples
/// ```
/// # use rsass::sass::Name;
/// assert_eq!(Name::from("foo-bar"), Name::from("foo_bar"));
/// assert_eq!(Name::from_static("foo_bar"), Name::from("foo-bar"));
/// ```
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Name {
    key: Cow<'static, str>,
}

impl Name {
    /// Key must not contain `-`.
    ///
    /// This function panics in debug mode if the key contains `-`.
    pub fn from_static(key: &'static str) -> Self {
        debug_assert!(!key.contains('-'));
        Self { key: key.into() }
    }

    /// Check if a name is "module.local".
    ///
    /// If so, return the module name and the local name as separate names.
    pub fn split_module(&self) -> Option<(String, Self)> {
        self.key.split_once('.').map(|(module, local)| {
            (
                module.replace('_', "-").to_string(),
                Self {
                    key: local.to_string().into(),
                },
            )
        })
    }
}

impl fmt::Display for Name {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        self.key.replace('_', "-").fmt(out)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.key.as_ref()
    }
}

impl From<String> for Name {
    fn from(key: String) -> Self {
        if key.contains('-') {
            Self {
                key: key.replace('-', "_").into(),
            }
        } else {
            Self { key: key.into() }
        }
    }
}

impl From<&str> for Name {
    fn from(key: &str) -> Self {
        Self {
            key: key.replace('-', "_").into(),
        }
    }
}
impl From<&String> for Name {
    fn from(key: &String) -> Self {
        let key: &str = key.as_ref();
        key.into()
    }
}

#[test]
fn test() {
    assert_eq!(Name::from_static("foo_bar"), "foo_bar".into());
    assert_eq!(Name::from_static("foo_bar"), "foo-bar".into());
    assert_eq!(Name::from_static("foo_bar"), "foo_bar".to_string().into());
    assert_eq!(Name::from_static("foo_bar"), "foo-bar".to_string().into());
}
