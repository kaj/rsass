use super::{check, CallError, FunctionMap};
use crate::css::Value;
use crate::value::ListSeparator;
use crate::Scope;

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:list");
    def!(f, append(list, val, separator = b"auto"), |s| {
        let (mut list, sep, bra) = get_list(s.get(name!(list))?);
        let sep = s
            .get_map(name!(separator), check_separator)?
            .or(sep)
            .unwrap_or(ListSeparator::Space);
        list.push(s.get(name!(val))?);
        Ok(Value::List(list, Some(sep), bra))
    });
    def!(f, index(list, value), |s| match s.get(name!(list))? {
        Value::List(v, _, _) => {
            let value = s.get(name!(value))?;
            for (i, v) in v.iter().enumerate() {
                if v == &value {
                    return Ok(Value::scalar(i + 1));
                }
            }
            Ok(Value::Null)
        }
        Value::Map(map) => match s.get(name!(value))? {
            Value::List(ref l, Some(ListSeparator::Space), _)
                if l.len() == 2 =>
            {
                for (i, &(ref k, ref v)) in map.iter().enumerate() {
                    if *k == l[0] && *v == l[1] {
                        return Ok(Value::scalar(i + 1));
                    }
                }
                Ok(Value::Null)
            }
            _ => Ok(Value::Null),
        },
        v => {
            if v == s.get(name!(value))? {
                Ok(Value::scalar(1))
            } else {
                Ok(Value::Null)
            }
        }
    });
    def!(f, is_bracketed(list), |s| Ok(match s.get(name!(list))? {
        Value::List(_, _, true) => Value::True,
        _ => Value::False,
    }));
    def!(
        f,
        join(list1, list2, separator = b"auto", bracketed = b"auto"),
        |s| {
            let (mut list1, sep1, bra1) = get_list(s.get(name!(list1))?);
            let (mut list2, sep2, _bra2) = get_list(s.get(name!(list2))?);
            let sep = s
                .get_map(name!(separator), check_separator)?
                .or(sep1)
                .or(sep2)
                .unwrap_or(ListSeparator::Space);
            list1.append(&mut list2);
            let bra = match s.get(name!(bracketed))? {
                Value::Literal(ref s) if s.value() == "auto" => bra1,
                b => b.is_true(),
            };
            Ok(Value::List(list1, Some(sep), bra))
        }
    );
    def!(f, length(list), |s| match s.get(name!(list))? {
        Value::ArgList(args) => Ok(Value::scalar(args.len() as i64)),
        Value::List(v, _, _) => Ok(Value::scalar(v.len() as i64)),
        Value::Map(m) => Ok(Value::scalar(m.len() as i64)),
        // A null value is considered eqivalent to an empty list
        Value::Null => Ok(Value::scalar(0)),
        // Any other value is a singleton list of that value
        _ => Ok(Value::scalar(1)),
    });
    def!(f, separator(list), |s| {
        let sep = match s.get(name!(list))? {
            Value::ArgList(..) => "comma",
            Value::List(_, Some(ListSeparator::Comma), _) => "comma",
            Value::List(_, Some(ListSeparator::Slash), _) => "slash",
            Value::Map(ref map) if !map.is_empty() => "comma",
            _ => "space",
        };
        Ok(sep.into())
    });
    def_va!(f, slash(elements), |s| {
        let list = s.get_map(name!(elements), check::va_list)?;
        if list.len() < 2 {
            return Err(CallError::msg(
                "At least two elements are required.",
            ));
        }
        Ok(Value::List(list, Some(ListSeparator::Slash), false))
    });
    def!(f, nth(list, n), |s| {
        match s.get(name!(list))? {
            Value::ArgList(arg) => {
                let n = s.get_map(name!(n), |v| index_of(v, arg.len()))?;
                Ok(arg.positional.get(n).cloned().unwrap_or_else(|| {
                    arg.named
                        .get_item(n - arg.positional.len())
                        .map(|(k, v)| {
                            Value::List(
                                vec![Value::from(k.as_ref()), v.clone()],
                                Some(ListSeparator::Space),
                                false,
                            )
                        })
                        .unwrap_or(Value::Null)
                }))
            }
            Value::List(list, _, _) => {
                let n = s.get_map(name!(n), |v| index_of(v, list.len()))?;
                Ok(list[n].clone())
            }
            Value::Map(map) => {
                let n = s.get_map(name!(n), |v| index_of(v, map.len()))?;
                if let Some(&(ref k, ref v)) = map.get_item(n) {
                    Ok(Value::List(
                        vec![k.clone(), v.clone()],
                        Some(ListSeparator::Space),
                        false,
                    ))
                } else {
                    Ok(Value::Null)
                }
            }
            v => s.get_map(name!(n), |v| index_of(v, 1)).map(|_| v),
        }
    });
    def!(f, set_nth(list, n, value), |s| {
        let (mut list, sep, bra) = get_list(s.get(name!(list))?);
        let i = s.get_map(name!(n), |v| index_of(v, list.len()))?;
        list[i] = s.get(name!(value))?;
        Ok(Value::List(list, sep, bra))
    });
    def_va!(f, zip(lists), |s| {
        let lists = s
            .get_map(name!(lists), check::va_list)?
            .into_iter()
            .map(|v| v.iter_items())
            .collect::<Vec<_>>();
        let len = lists.iter().map(|v| v.len()).min().unwrap_or(0);
        let result = (0..len)
            .map(|i| {
                let items = lists.iter().map(|v| v[i].clone()).collect();
                Value::List(items, Some(ListSeparator::Space), false)
            })
            .collect();
        Ok(Value::List(result, Some(ListSeparator::Comma), false))
    });
    f
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        (name!(append), name!(append)),
        (name!(index), name!(index)),
        (name!(is_bracketed), name!(is_bracketed)),
        (name!(join), name!(join)),
        (name!(length), name!(length)),
        (name!(list_separator), name!(separator)),
        (name!(nth), name!(nth)),
        (name!(set_nth), name!(set_nth)),
        (name!(zip), name!(zip)),
    ] {
        global.insert(gname.clone(), m.get_lfunction(lname));
    }
}

fn check_separator(v: Value) -> Result<Option<ListSeparator>, String> {
    match String::try_from(v)?.as_ref() {
        "comma" => Ok(Some(ListSeparator::Comma)),
        "slash" => Ok(Some(ListSeparator::Slash)),
        "space" => Ok(Some(ListSeparator::Space)),
        "auto" => Ok(None),
        _ => {
            Err("Must be \"space\", \"comma\", \"slash\", or \"auto\"."
                .into())
        }
    }
}

fn get_list(value: Value) -> (Vec<Value>, Option<ListSeparator>, bool) {
    match value {
        Value::ArgList(args) => {
            let mut vec = args.positional;
            // I'm not sure that including the named arguments after the
            // positional is the right thing to do here.
            // Maybe return an error instead?
            vec.extend(args.named.into_iter().map(|(k, v)| {
                Value::List(
                    vec![Value::from(k.as_ref()), v],
                    Some(ListSeparator::Space),
                    false,
                )
            }));
            (vec, Some(ListSeparator::Comma), false)
        }
        Value::List(v, s, bra) => (v, s, bra),
        Value::Map(map) => {
            if map.is_empty() {
                (vec![], None, false)
            } else {
                (
                    map.iter()
                        .map(|&(ref k, ref v)| {
                            Value::List(
                                vec![k.clone(), v.clone()],
                                Some(ListSeparator::Space),
                                false,
                            )
                        })
                        .collect(),
                    Some(ListSeparator::Comma),
                    false,
                )
            }
        }
        v => (vec![v], None, false),
    }
}

fn index_of(v: Value, len: usize) -> Result<usize, String> {
    let n = check::unitless_int(v)?;
    if n.is_positive() && n as usize <= len {
        Ok((n - 1) as usize)
    } else if n.is_negative() && n >= -(len as i64) {
        Ok((len as i64 + n) as usize)
    } else if n == 0 {
        Err("List index may not be 0.".into())
    } else {
        Err(format!(
            "Invalid index {} for a list with {} elements.",
            n, len
        ))
    }
}

#[cfg(test)]
mod test {
    // Append function tests from
    // http://sass-lang.com/documentation/Sass/Script/Functions.html
    #[test]
    fn append_a() {
        check_val("append(10px 20px, 30px);", "10px 20px 30px")
    }
    #[test]
    fn append_b() {
        check_val("append((blue, red), green);", "blue, red, green")
    }
    #[test]
    fn append_c() {
        // the documentation is incorrect on this one, libsass does not
        // add parentheses in this case.
        check_val("append(10px 20px, 30px 40px);", "10px 20px 30px 40px")
    }
    #[test]
    fn append_d() {
        check_val("append(10px, 20px, comma);", "10px, 20px")
    }
    #[test]
    fn append_e() {
        check_val("append((blue, red), green, space);", "blue red green")
    }

    mod join {
        //! From `sass-spec/spec/core_functions/join/valid`

        use super::check_val;

        #[test]
        fn both_bracketed() {
            check_val("join([foo bar], [baz bang]);", "[foo bar baz bang]")
        }
        #[test]
        fn first_bracketed() {
            check_val("join([foo bar], baz bang);", "[foo bar baz bang]")
        }
        #[test]
        fn second_bracketed() {
            check_val("join(foo bar, [baz bang]);", "foo bar baz bang")
        }
        #[test]
        fn bracketed_true() {
            check_val("join(foo, bar, $bracketed: true);", "[foo bar]")
        }
        #[test]
        fn bracketed_false() {
            check_val("join([foo], [bar], $bracketed: false);", "foo bar")
        }
        #[test]
        fn separator_and_bracketed() {
            check_val(
                "join(foo, bar, $separator: comma, $bracketed: true);",
                "[foo, bar]",
            )
        }
        #[test]
        fn bracketed_and_separator() {
            check_val(
                "join(foo, bar, $bracketed: true, $separator: comma);",
                "[foo, bar]",
            )
        }
        #[test]
        fn separator_and_bracketed_positional() {
            check_val("join(foo, bar, comma, true);", "[foo, bar]")
        }
        #[test]
        fn unusual_bracketed_type() {
            check_val("join(foo, bar, $bracketed: foo);", "[foo bar]")
        }
        #[test]
        fn bracketed_null() {
            check_val("join([foo], [bar], $bracketed: null);", "foo bar")
        }
    }

    #[test]
    fn is_bracketed() {
        check_val("is_bracketed([foo]);", "true");
    }

    #[test]
    fn zip() {
        check_val(
            "zip(1px 1px 3px, solid dashed solid, red green blue);",
            "1px solid red, 1px dashed green, 3px solid blue",
        )
    }

    fn check_val(src: &str, correct: &str) {
        use crate::variablescope::test::do_evaluate;
        assert_eq!(do_evaluate(&[], src.as_bytes()), correct)
    }
}
