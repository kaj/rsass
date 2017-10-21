use super::{Error, SassFunction};
use css::Value;
use std::collections::BTreeMap;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, map_get(map, key), |s| match s.get("map") {
        Value::Map(m) => Ok(m.get(&s.get("key")).cloned().unwrap_or(Value::Null)),
        // An empty map and an empty list looks the same
        Value::List(ref l, ..) if l.is_empty() => Ok(Value::Null),
        v => Err(Error::badarg("map", &v)),
    });
}

#[cfg(test)]
mod test {
    // http://sass-lang.com/documentation/Sass/Script/Functions.html

    mod map_get {
        use super::check_val;

        #[test]
        fn a() {
            check_val("map-get((\"foo\": 1, \"bar\": 2), \"foo\");", "1")
        }
        #[test]
        fn b() {
            check_val("map-get((\"foo\": 1, \"bar\": 2), \"bar\");", "2")
        }
        #[test]
        fn c() {
            check_val("map-get((\"foo\": 1, \"bar\": 2), \"baz\");", "")
        }
    }


    fn check_val(src: &str, correct: &str) {
        use variablescope::test::do_evaluate;
        assert_eq!(do_evaluate(&[], src.as_bytes()), correct)
    }
}
