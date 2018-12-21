use super::{Error, SassFunction};
use css::Value;
use std::collections::BTreeMap;
use value::{ListSeparator, Quotes};

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, length(list), |s| match s.get("list") {
        Value::List(v, _, _) => Ok(Value::scalar(v.len() as isize)),
        Value::Map(m) => Ok(Value::scalar(m.len() as isize)),
        // Any other value is a singleton list of that value
        _ => Ok(Value::scalar(1)),
    });
    def!(f, nth(list, n), |s| {
        let n = s.get("n").integer_value()?;
        match s.get("list") {
            Value::List(list, _, _) => {
                Ok(list[list_index(n, &list)?].clone())
            }
            Value::Map(map) => {
                let n = rust_index(n, map.len())?;
                if let Some(&(ref k, ref v)) = map.get_item(n) {
                    Ok(Value::List(
                        vec![k.clone(), v.clone()],
                        ListSeparator::Space,
                        false,
                    ))
                } else {
                    Ok(Value::Null)
                }
            }
            v => Err(Error::badarg("list", &v)),
        }
    });
    def!(f, set_nth(list, n, value), |s| {
        let n = s.get("n").integer_value()?;
        let (mut list, sep, bra) = get_list(s.get("list"));
        let i = list_index(n, &list)?;
        list[i] = s.get("value");
        Ok(Value::List(list, sep.unwrap_or(ListSeparator::Space), bra))
    });
    def!(
        f,
        join(list1, list2, separator = b"auto", bracketed = b"auto"),
        |s| {
            let (mut list1, sep1, bra1) = get_list(s.get("list1"));
            let (mut list2, sep2, _bra2) = get_list(s.get("list2"));
            let separator = match s.get("separator") {
                Value::Literal(ref sep, _)
                    if sep.to_lowercase() == "comma" =>
                {
                    ListSeparator::Comma
                }
                Value::Literal(ref sep, _)
                    if sep.to_lowercase() == "space" =>
                {
                    ListSeparator::Space
                }
                Value::Literal(ref sep, _)
                    if sep.to_lowercase() == "auto" =>
                {
                    sep1.or(sep2).unwrap_or(ListSeparator::Space)
                }
                ref other => {
                    Err(Error::badarg("'comma', 'space', or 'auto'", other))
                }
            };
            list1.append(&mut list2);
            let bra = match s.get("bracketed") {
                Value::Literal(ref s, _) if s == "auto" => bra1,
                b => b.is_true(),
            };
            Ok(Value::List(list1, separator, bra))
        }
    );
    def!(f, append(list, val, separator), |s| {
        let (mut list, sep, bra) = get_list(s.get("list"));
        let sep = match (s.get("separator"), sep) {
            (Value::Literal(ref s, _), _) if s == "comma" => {
                ListSeparator::Comma
            }
            (Value::Literal(ref s, _), _) if s == "space" => {
                ListSeparator::Space
            }
            (_, s) => s.unwrap_or(ListSeparator::Space),
        };
        list.push(s.get("val"));
        Ok(Value::List(list, sep, bra))
    });
    def_va!(f, zip(lists), |s| match s.get("lists") {
        Value::List(v, _, _) => {
            let lists =
                v.into_iter().map(|v| v.iter_items()).collect::<Vec<_>>();
            let len = lists.iter().map(|v| v.len()).min().unwrap_or(0);
            let result = (0..len)
                .map(|i| {
                    let items = lists.iter().map(|v| v[i].clone()).collect();
                    Value::List(items, ListSeparator::Space, false)
                })
                .collect();
            Ok(Value::List(result, ListSeparator::Comma, false))
        }
        v => Err(Error::badarg("list", &v)),
    });
    def!(f, index(list, value), |s| match s.get("list") {
        Value::List(v, _, _) => {
            let value = s.get("value");
            for (i, v) in v.iter().enumerate() {
                if v == &value {
                    return Ok(Value::scalar(i as isize + 1));
                }
            }
            Ok(Value::Null)
        }
        Value::Map(map) => match s.get("value") {
            Value::List(ref l, ListSeparator::Space, _) if l.len() == 2 => {
                for (i, &(ref k, ref v)) in map.iter().enumerate() {
                    if *k == l[0] && *v == l[1] {
                        return Ok(Value::scalar(i as isize + 1));
                    }
                }
                Ok(Value::Null)
            }
            _ => Ok(Value::Null),
        },
        v => Err(Error::badarg("list", &v)),
    });
    def!(f, list_separator(list), |s| Ok(Value::Literal(
        match s.get("list") {
            Value::List(_, ListSeparator::Comma, _) => "comma",
            _ => "space",
        }
        .into(),
        Quotes::None
    )));
    def!(f, is_bracketed(list), |s| Ok(match s.get("list") {
        Value::List(_, _, true) => Value::True,
        _ => Value::False,
    }));
}

fn get_list(value: Value) -> (Vec<Value>, Option<ListSeparator>, bool) {
    match value {
        Value::List(v, s, bra) => (v, Some(s), bra),
        Value::Map(map) => (
            map.iter()
                .map(|&(ref k, ref v)| {
                    Value::List(
                        vec![k.clone(), v.clone()],
                        ListSeparator::Space,
                        false,
                    )
                })
                .collect(),
            Some(ListSeparator::Comma),
            false,
        ),
        v => (vec![v], None, false),
    }
}

fn list_index(n: isize, list: &[Value]) -> Result<usize, Error> {
    let len = list.len();
    rust_index(n, len)
}

fn rust_index(n: isize, len: usize) -> Result<usize, Error> {
    if n > 0 && n as usize <= len {
        Ok((n - 1) as usize)
    } else if n < 0 && n >= -(len as isize) {
        Ok((len as isize + n) as usize)
    } else {
        let msg =
            format!("Expected index for list of length {}, got {}", len, n);
        Err(Error::BadArguments(msg))
    }
}

#[cfg(test)]
mod test {
    // Append fuction tests from
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
        use variablescope::test::do_evaluate;
        assert_eq!(do_evaluate(&[], src.as_bytes()), correct)
    }
}
