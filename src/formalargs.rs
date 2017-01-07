use parseutil::{ignore_comments, name, spacelike};
use std::default::Default;
use std::fmt;
use valueexpression::{Value, space_list};
use variablescope::{Scope, ScopeImpl};

/// The declared arguments of a mixin or function declaration.
///
/// The arguments are ordered (so they have a position).
/// Each argument also has a name and may have a default value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormalArgs(Vec<(String, Option<Value>)>);

impl FormalArgs {
    #[cfg(test)]
    pub fn new(a: Vec<(String, Option<Value>)>) -> Self {
        FormalArgs(a)
    }

    pub fn eval<'a>(&self,
                    scope: &'a mut Scope,
                    args: &CallArgs)
                    -> ScopeImpl<'a> {
        let mut argscope = ScopeImpl::sub(scope);
        for (i, &(ref name, ref default)) in self.0.iter().enumerate() {
            args.0
                .iter()
                .find(|&&(ref k, ref _v)| k.as_ref() == Some(name))
                .or_else(|| args.0.get(i))
                .map(|&(ref _k, ref v)| v)
                .or_else(|| default.as_ref())
                .map(|value| argscope.define(&name, &value, false));
        }
        argscope
    }
}

/// the actual arguments of a function or mixin call.
///
/// Each argument has a Value.  Arguments may be named.
/// If the optional name is None, the argument is positional.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallArgs(Vec<(Option<String>, Value)>);

impl CallArgs {
    #[cfg(test)]
    pub fn new(v: Vec<(Option<String>, Value)>) -> Self {
        CallArgs(v)
    }

    pub fn xyzzy(&self, scope: &mut Scope) -> Self {
        CallArgs(self.0
            .iter()
            .map(|&(ref n, ref v)| (n.clone(), scope.evaluate(&v)))
            .collect())
    }
}

impl Default for CallArgs {
    fn default() -> Self {
        CallArgs(vec![])
    }
}

impl fmt::Display for CallArgs {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let t = self.0
            .iter()
            .map(|&(ref k, ref v)| {
                if let &Some(ref k) = k {
                    format!("${}: {}", k, v)
                } else {
                    format!("{}", v)
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        write!(out, "{}", t)
    }
}

named!(pub formal_args<FormalArgs>,
       chain!(tag!("(") ~ spacelike? ~
              args: separated_list!(
                  chain!(tag!(",") ~ spacelike?, || ()),
                  chain!(tag!("$") ~ name: name ~
                         d: opt!(chain!(tag!(":") ~ spacelike? ~
                                        d: space_list ~ spacelike?,
                                        || d)),
                         || (name, d))) ~
              tag!(")"),
              || FormalArgs(args)));

named!(pub call_args<CallArgs>,
       chain!(tag!("(") ~
              args: separated_list!(
                  chain!(tag!(",") ~ spacelike?, || ()),
                  alt_complete!(
                      chain!(tag!("$") ~
                             name: name ~
                             ignore_comments ~
                             tag!(":") ~
                             ignore_comments ~
                             val: space_list ~
                             ignore_comments,
                             || (Some(name), val)) |
                      chain!(e: space_list, || (None, e)))) ~
              tag!(")"),
              || CallArgs(args)
              ));
