use super::{Error, SassFunction};
use css::Value;
use std::collections::BTreeMap;
use value::{ListSeparator, Quotes};

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, length(list), |s| match s.get("list") {
        Value::List(v, _, _) => Ok(Value::scalar(v.len() as isize)),
        v => Err(Error::badarg("list", &v)),
    });
    def!(f, nth(list, n), |s| {
        let n = s.get("n").integer_value()?;
        match s.get("list") {
            Value::List(list, _, _) => Ok(list[rust_index(n, &list)?].clone()),
            v => Err(Error::badarg("list", &v)),
        }
    });
    def!(f, set_nth(list, n, value), |s| {
        let n = s.get("n").integer_value()?;
        let (mut list, sep, bra) = match s.get("list") {
            Value::List(v, s, bra) => (v, s, bra),
            v => (vec![v], ListSeparator::Space, false),
        };
        let i = rust_index(n, &list)?;
        list[i] = s.get("value");
        Ok(Value::List(list, sep, bra))
    });
    def!(f, join(list1, list2, separator = b"auto", bracketed = b"auto"), |s| {
        let (mut list1, sep1, bra1) = match s.get("list1") {
            Value::List(list, sep, bra) => (list, Some(sep), bra),
            v => (vec![v], None, false),
        };
        let (mut list2, sep2, _bra2) = match s.get("list2") {
            Value::List(list, sep, bra) => (list, Some(sep), bra),
            v => (vec![v], None, false),
        };
        let separator = match s.get("separator") {
            Value::Literal(sep, _) => {
                let sep = sep.to_lowercase();
                if sep == "comma" {
                    ListSeparator::Comma
                } else if sep == "space" {
                    ListSeparator::Space
                } else if sep == "auto" {
                    sep1.or(sep2).unwrap_or(ListSeparator::Space)
                } else {
                    return Err(Error::badarg("'comma', 'space', or 'auto'",
                                             &Value::Literal(sep, Quotes::Double)));
                }
            }
            other => return Err(Error::badarg("'comma', 'space', or 'auto'", &other)),
        };
        list1.append(&mut list2);
        let bra = match s.get("bracketed") {
            Value::Literal(ref s, _) if s == "auto" => bra1,
            b => b.is_true(),
        };
        Ok(Value::List(list1, separator, bra))
    });
    def!(f, append(list, val, separator), |s| {
        let (mut list, sep, bra) = match s.get("list") {
            Value::List(v, s, bra) => (v, Some(s), bra),
            v => (vec![v], None, false),
        };
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
    def!(f, index(list, value), |s| {
        let v = match s.get("list") {
            Value::List(v, _, _) => v,
            v => return Err(Error::badarg("list", &v)),
        };
        let value = s.get("value");
        for (i, v) in v.iter().enumerate() {
            if v == &value {
                return Ok(Value::scalar(i as isize + 1));
            }
        }
        Ok(Value::Null)
    });
    def!(f, list_separator(list), |s| {
        Ok(Value::Literal(match s.get("list") {
                                  Value::List(_, ListSeparator::Comma, _) => {
                                      "comma"
                                  }
                                  _ => "space",
                              }
                              .into(),
                          Quotes::None))
    });
}

fn rust_index(n: isize, list: &[Value]) -> Result<usize, Error> {
    let len = list.len();
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
    use variablescope::test::do_evaluate;

    // Append fuction tests from
    // http://sass-lang.com/documentation/Sass/Script/Functions.html
    #[test]
    fn append_a() {
        assert_eq!(do_evaluate(&[], b"append(10px 20px, 30px);"),
                   "10px 20px 30px")
    }
    #[test]
    fn append_b() {
        assert_eq!(do_evaluate(&[], b"append((blue, red), green);"),
                   "blue, red, green")
    }
    #[test]
    fn append_c() {
        // the documentation is incorrect on this one, libsass does not
        // add parentheses in this case.
        assert_eq!(do_evaluate(&[], b"append(10px 20px, 30px 40px);"),
                   "10px 20px 30px 40px")
    }
    #[test]
    fn append_d() {
        assert_eq!(do_evaluate(&[], b"append(10px, 20px, comma);"),
                   "10px, 20px")
    }
    #[test]
    fn append_e() {
        assert_eq!(do_evaluate(&[], b"append((blue, red), green, space);"),
                   "blue red green")
    }

    mod join {
        //! From `sass-spec/spec/core_functions/join/valid`

        #[test]
        fn both_bracketed() {
            check_val("join([foo bar], [baz bang]);",
                      "[foo bar baz bang]")
        }
        #[test]
        fn first_bracketed() {
            check_val("join([foo bar], baz bang);",
                      "[foo bar baz bang]")
        }
        #[test]
        fn second_bracketed() {
            check_val("join(foo bar, [baz bang]);",
                      "foo bar baz bang")
        }
        #[test]
        fn bracketed_true() {
            check_val("join(foo, bar, $bracketed: true);",
                      "[foo bar]")
        }
        #[test]
        fn bracketed_false() {
            check_val("join([foo], [bar], $bracketed: false);",
                      "foo bar")
        }
        #[test]
        fn separator_and_bracketed() {
            check_val("join(foo, bar, $separator: comma, $bracketed: true);",
                      "[foo, bar]")
        }
        #[test]
        fn bracketed_and_separator() {
            check_val("join(foo, bar, $bracketed: true, $separator: comma);",
                      "[foo, bar]")
        }
        #[test]
        fn separator_and_bracketed_positional() {
            check_val("join(foo, bar, comma, true);",
                      "[foo, bar]")
        }
        #[test]
        fn unusual_bracketed_type() {
            check_val("join(foo, bar, $bracketed: foo);",
                      "[foo bar]")
        }
        #[test]
        fn bracketed_null() {
            check_val("join([foo], [bar], $bracketed: null);",
                      "foo bar")
        }
        fn check_val(src: &str, correct: &str) {
            use variablescope::test::do_evaluate;
            assert_eq!(do_evaluate(&[], src.as_bytes()), correct)
        }
    }
}
