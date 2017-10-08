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
}
