use parseutil::{ignore_comments, name, opt_spacelike};
use std::default::Default;
use std::fmt;
use valueexpression::{Value, space_list};
use variablescope::{Scope, ScopeImpl};

/// The declared arguments of a mixin or function declaration.
///
/// The arguments are ordered (so they have a position).
/// Each argument also has a name and may have a default value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormalArgs(Vec<(String, Value)>);

impl FormalArgs {
    pub fn new(a: Vec<(String, Value)>) -> Self {
        FormalArgs(a)
    }

    pub fn eval<'a>(&self,
                    scope: &'a mut Scope,
                    args: &CallArgs)
                    -> ScopeImpl<'a> {
        let mut argscope = ScopeImpl::sub(scope);
        for (i, &(ref name, ref default)) in self.0.iter().enumerate() {
            let value = args.0
                .iter()
                .find(|&&(ref k, ref _v)| k.as_ref() == Some(name))
                .map(|&(ref _k, ref v)| v)
                .or_else(|| match args.0.get(i) {
                             Some(&(None, ref v)) => Some(v),
                             _ => None,
                         })
                .unwrap_or(default);
            argscope.define(name, value, false);
        }
        argscope
    }
}

impl Default for FormalArgs {
    fn default() -> Self {
        FormalArgs::new(vec![])
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
                     .map(|&(ref n, ref v)| (n.clone(), scope.evaluate(v)))
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
            .map(|kv| match *kv {
                     (Some(ref k), ref v) => format!("${}: {}", k, v),
                     (None, ref v) => format!("{}", v),
                 })
            .collect::<Vec<_>>()
            .join(", ");
        write!(out, "{}", t)
    }
}

named!(pub formal_args<FormalArgs>,
       delimited!(preceded!(tag!("("), opt_spacelike),
                  map!(separated_list!(
                      preceded!(tag!(","), opt_spacelike),
                      do_parse!(tag!("$") >> name: name >>
                                d: opt!(do_parse!(
                                    tag!(":") >> opt_spacelike >>
                                        d: space_list >> opt_spacelike >>
                                        (d))) >>
                                (name, d.unwrap_or(Value::Null)))),
                       |v| FormalArgs(v)),
                  tag!(")")));

named!(pub call_args<CallArgs>,
       delimited!(
           tag!("("),
           map!(separated_list!(
               preceded!(tag!(","), opt_spacelike),
               pair!(opt!(delimited!(
                        tag!("$"),
                        map!(name, |n: String| n.replace("-", "_")),
                        preceded!(ignore_comments,
                                  tag!(":")))),
                     delimited!(ignore_comments,
                                space_list,
                                ignore_comments))),
                |args| CallArgs(args)),
           tag!(")")));
