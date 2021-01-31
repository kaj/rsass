use std::borrow::Cow;

/// A sass name, used to idenify functions, variables, mixins, etc.
///
/// A `-` and a `_` is considered equal in a name, both represented by a `_`.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Name {
    key: Cow<'static, str>,
}

impl Name {
    /// Key must not contain `-`.
    pub fn from_static(key: &'static str) -> Name {
        debug_assert!(!key.contains('-'));
        Name { key: key.into() }
    }

    pub fn split_module(&self) -> Option<(Name, Name)> {
        let mut parts = self.key.splitn(2, '.');
        if let (Some(module), Some(local)) = (parts.next(), parts.next()) {
            Some((
                Name {
                    key: module.to_string().into(),
                },
                Name {
                    key: local.to_string().into(),
                },
            ))
        } else {
            None
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.key.as_ref()
    }
}

impl From<String> for Name {
    fn from(key: String) -> Name {
        if key.contains('-') {
            Name {
                key: key.replace('-', "_").into(),
            }
        } else {
            Name { key: key.into() }
        }
    }
}

impl From<&str> for Name {
    fn from(key: &str) -> Name {
        Name {
            key: key.replace('-', "_").into(),
        }
    }
}
impl From<&String> for Name {
    fn from(key: &String) -> Name {
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
