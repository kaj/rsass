use formalargs::{CallArgs, FormalArgs};
use parser::util::{ignore_comments, name, opt_spacelike};
use parser::value::{extended_literal, space_list};
use value::Value;

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
                 (FormalArgs::new(v, va.is_some()))));

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
                     alt!(space_list |
                          extended_literal |
                          delimited!(ignore_comments,
                                     space_list,
                                     ignore_comments)))),
                |args| CallArgs::new(args)),
           tag!(")")));
