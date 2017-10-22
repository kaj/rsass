use super::{Error, SassFunction};
use css::Value;
use std::collections::BTreeMap;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, map_get(map, key), |s| {
        Ok(get_map(s.get("map"))?
               .get(&s.get("key"))
               .cloned()
               .unwrap_or(Value::Null))
    });
    def!(f, map_merge(map1, map2), |s| {
        let mut map1 = get_map(s.get("map1"))?;
        let map2 = get_map(s.get("map2"))?;
        for (key, value) in map2 {
            map1.insert(key, value);
        }
        Ok(Value::Map(map1))
    });
    def!(f, map_has_key(map, key), |s| {
        let map = get_map(s.get("map"))?;
        Ok(Value::bool(map.contains_key(&s.get("key"))))
    });
}

fn get_map(v: Value) -> Result<BTreeMap<Value, Value>, Error> {
    match v {
        Value::Map(m) => Ok(m),
        // An empty map and an empty list looks the same
        Value::List(ref l, ..) if l.is_empty() => Ok(BTreeMap::new()),
        v => Err(Error::badarg("map", &v)),
    }
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

    mod map_has_key {
        use super::check_val;

        #[test]
        fn a() {
            check_val("map-has-key((\"foo\": 1, \"bar\": 2), \"foo\");", "true")
        }
        #[test]
        fn b() {
            check_val("map-has-key((\"foo\": 1, \"bar\": 2), \"baz\");",
                      "false")
        }
    }

    fn check_val(src: &str, correct: &str) {
        use variablescope::test::do_evaluate;
        assert_eq!(do_evaluate(&[], src.as_bytes()), correct)
    }
}
