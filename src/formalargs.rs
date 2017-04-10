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
pub struct FormalArgs(Vec<(String, Value)>, bool);

impl FormalArgs {
    pub fn new(a: Vec<(String, Value)>) -> Self {
        FormalArgs(a, false)
    }

    pub fn eval<'a>(&self,
                    scope: &'a mut Scope,
                    args: &CallArgs)
                    -> ScopeImpl<'a> {
        let mut argscope = ScopeImpl::sub(scope);
        let n = self.0.len();
        for (i, &(ref name, ref default)) in self.0.iter().enumerate() {
            if let Some(value) = args.0
                   .iter()
                   .find(|&&(ref k, ref _v)| k.as_ref() == Some(name))
                   .map(|&(ref _k, ref v)| v) {
                argscope.define(name, value, false);
            } else if self.1 && i + 1 == n && args.0.len() > n {
                let args =
                    args.0[i..].iter().map(|&(_, ref v)| v.clone()).collect();
                argscope.define(name, &Value::MultiComma(args), false);
            } else {
                argscope.define(name,
                                match args.0.get(i) {
                                    Some(&(None, ref v)) => v,
                                    _ => default,
                                },
                                false);
            }
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
       do_parse!(tag!("(") >> opt_spacelike >>
                 v: separated_list!(
                     preceded!(tag!(","), opt_spacelike),
                     do_parse!(tag!("$") >> name: name >>
                               d: opt!(do_parse!(
                                   opt_spacelike >>
                                   tag!(":") >> opt_spacelike >>
                                   d: space_list >> opt_spacelike >>
                                   (d))) >>
                               (name.replace('-', "_"),
                                d.unwrap_or(Value::Null)))) >>
                 va: opt!(tag!("...")) >> opt_spacelike >>
                 tag!(")") >>
                 (FormalArgs(v, va.is_some()))));

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
