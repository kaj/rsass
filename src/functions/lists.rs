use functions::{SassFunction, badarg};
use std::collections::BTreeMap;
use valueexpression::Value;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, length(list), |s| match s.get("list") {
        Value::MultiComma(v) => Ok(Value::scalar(v.len() as isize)),
        Value::MultiSpace(v) => Ok(Value::scalar(v.len() as isize)),
        v => Err(badarg("list", &v)),
    });
    def!(f, nth(list, n), |s| {
        let n = match s.get("n") {
            Value::Numeric(val, _, _) if val.denom() == &1 => val.to_integer(),
            x => return Err(badarg("integer", &x)),
        };
        let list = match s.get("list") {
            Value::MultiComma(v) => v,
            Value::MultiSpace(v) => v,
            v => return Err(badarg("list", &v)),
        };
        Ok(list[n as usize - 1].clone())
    });
    def!(f, append(list, val, separator), |s| {
        let (comma_sep, mut list) = match s.get("list") {
            Value::MultiComma(v) => (true, v),
            Value::MultiSpace(v) => (false, v),
            v => (false, vec![v]),
        };
        let comma_sep = match (s.get("separator"), comma_sep) {
            (Value::Literal(ref s, _), _) if s == "comma" => true,
            (Value::Literal(ref s, _), _) if s == "space" => false,
            (_, s) => s,
        };
        list.push(match s.get("val") {
                      Value::MultiSpace(v) => {
                          Value::Paren(Box::new(Value::MultiSpace(v)))
                      }
                      v => v,
                  });
        Ok(if comma_sep {
               Value::MultiComma(list)
           } else {
               Value::MultiSpace(list)
           })
    });
    def!(f, index(list, value), |s| {
        let v = match s.get("list") {
            Value::MultiComma(v) => v,
            Value::MultiSpace(v) => v,
            v => return Err(badarg("list", &v)),
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
