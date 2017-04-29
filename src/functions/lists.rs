use super::{Error, SassFunction};
use std::collections::BTreeMap;
use valueexpression::{ListSeparator, Value};

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, length(list), |s| match s.get("list") {
        Value::List(v, _) => Ok(Value::scalar(v.len() as isize)),
        v => Err(Error::badarg("list", &v)),
    });
    def!(f, nth(list, n), |s| {
        let n = match s.get("n") {
            Value::Numeric(val, _, _) if val.denom() == &1 => val.to_integer(),
            x => return Err(Error::badarg("integer", &x)),
        };
        match s.get("list") {
            Value::List(list, _) => Ok(list[n as usize - 1].clone()),
            v => Err(Error::badarg("list", &v)),
        }
    });
    def!(f, append(list, val, separator), |s| {
        let (mut list, sep) = match s.get("list") {
            Value::List(v, s) => (v, Some(s)),
            v => (vec![v], None),
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
        list.push(match s.get("val") {
                      Value::List(v, s) => {
                          Value::Paren(Box::new(Value::List(v, s)))
                      }
                      v => v,
                  });
        Ok(Value::List(list, sep))
    });
    def!(f, index(list, value), |s| {
        let v = match s.get("list") {
            Value::List(v, _) => v,
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
        assert_eq!(do_evaluate(&[], b"append(10px 20px, 30px 40px);"),
                   "10px 20px (30px 40px)")
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
