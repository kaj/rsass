use super::{check, get_checked, get_integer, Error, FunctionMap, Name};
use crate::css::Value;
use crate::value::{ListSeparator, Quotes};
use crate::Scope;

pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:list");
    def!(f, append(list, val, separator = b"auto"), |s| {
        let (mut list, sep, bra) = get_list(s.get("list")?);
        let sep = get_checked(s, name!(separator), check_separator)?
            .or(sep)
            .unwrap_or(ListSeparator::Space);
        list.push(s.get("val")?);
        Ok(Value::List(list, Some(sep), bra))
    });
    def!(f, index(list, value), |s| match s.get("list")? {
        Value::List(v, _, _) => {
            let value = s.get("value")?;
            for (i, v) in v.iter().enumerate() {
                if v == &value {
                    return Ok(Value::scalar(i + 1));
                }
            }
            Ok(Value::Null)
        }
        Value::Map(map) => match s.get("value")? {
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
            if v == s.get("value")? {
                Ok(Value::scalar(1))
            } else {
                Ok(Value::Null)
            }
        }
    });
    def!(f, is_bracketed(list), |s| Ok(match s.get("list")? {
        Value::List(_, _, true) => Value::True,
        _ => Value::False,
    }));
    def!(
        f,
        join(list1, list2, separator = b"auto", bracketed = b"auto"),
        |s| {
            let (mut list1, sep1, bra1) = get_list(s.get("list1")?);
            let (mut list2, sep2, _bra2) = get_list(s.get("list2")?);
            let sep = get_checked(s, name!(separator), check_separator)?
                .or(sep1)
                .or(sep2)
                .unwrap_or(ListSeparator::Space);
            list1.append(&mut list2);
            let bra = match s.get("bracketed")? {
                Value::Literal(ref s, _) if s == "auto" => bra1,
                b => b.is_true(),
            };
            Ok(Value::List(list1, Some(sep), bra))
        }
    );
    def!(f, length(list), |s| match s.get("list")? {
        Value::List(v, _, _) => Ok(Value::scalar(v.len() as i64)),
        Value::Map(m) => Ok(Value::scalar(m.len() as i64)),
        // A null value is considered eqivalent to an empty list
        Value::Null => Ok(Value::scalar(0)),
        // Any other value is a singleton list of that value
        _ => Ok(Value::scalar(1)),
    });
    def!(f, separator(list), |s| Ok(Value::Literal(
        match s.get("list")? {
            Value::List(_, Some(ListSeparator::Comma), _) => "comma",
            Value::Map(ref map) if map.len() == 0 => "space",
            Value::Map(_) => "comma",
            _ => "space",
        }
        .into(),
        Quotes::None
    )));
    def!(f, nth(list, n), |s| {
        let n = get_integer(s, name!(n))?;
        match s.get("list")? {
            Value::List(list, _, _) => {
                Ok(list[list_index(n, &list, name!(n))?].clone())
            }
            Value::Map(map) => {
                let n = rust_index(n, map.len(), name!(n))?;
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
            v => Ok(if n == 1 { v } else { Value::Null }),
        }
    });
    def!(f, set_nth(list, n, value), |s| {
        let (mut list, sep, bra) = get_list(s.get("list")?);
        let n = get_integer(s, name!(n))?;
        let i = list_index(n, &list, name!(n))?;
        list[i] = s.get("value")?;
        Ok(Value::List(list, sep, bra))
    });
    def_va!(f, zip(lists), |s| {
        // TODO: A single argument is just the first list, unless its
        // names "lists".
        // A possible implementation may be a zip(first, lists) style, so I
        // the first unnamed argument in a separate variable.
        // But that would leed to problems with to unnamed arguments.
        // We probably need to actually make a difference between
        // named an positional arguments!
        let (v, s, b) = get_list(s.get("lists")?);
        let v = if b { vec![Value::List(v, s, b)] } else { v };
        let lists = v.into_iter().map(|v| v.iter_items()).collect::<Vec<_>>();
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
        global.insert(gname.clone(), m.get_function(&lname).unwrap().clone());
    }
}

fn check_separator(v: Value) -> Result<Option<ListSeparator>, String> {
    match check::string(v)?.0.as_ref() {
        "comma" => Ok(Some(ListSeparator::Comma)),
        "space" => Ok(Some(ListSeparator::Space)),
        "auto" => Ok(None),
        _ => Err("Must be \"space\", \"comma\", or \"auto\"".into()),
    }
}

fn get_list(value: Value) -> (Vec<Value>, Option<ListSeparator>, bool) {
    match value {
        Value::List(v, s, bra) => (v, s, bra),
        Value::Map(map) => {
            if map.len() == 0 {
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

fn list_index(n: i64, list: &[Value], name: Name) -> Result<usize, Error> {
    let len = list.len();
    rust_index(n, len, name)
}

fn rust_index(n: i64, len: usize, name: Name) -> Result<usize, Error> {
    if n > 0 && n as usize <= len {
        Ok((n - 1) as usize)
    } else if n < 0 && n >= -(len as i64) {
        Ok((len as i64 + n) as usize)
    } else {
        let msg = if n == 0 {
            "List index may not be 0".into()
        } else {
            format!("Invalid index {} for a list with {} elements", n, len)
        };
        Err(Error::BadArgument(name, msg))
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
