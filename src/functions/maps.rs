use super::{Error, SassFunction};
use css::Value;
use ordermap::OrderMap;
use std::collections::BTreeMap;
use value::ListSeparator;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, map_get(map, key), |s| Ok(get_map(s.get("map"))?
        .get(&s.get("key"))
        .cloned()
        .unwrap_or(Value::Null)));
    def!(f, map_merge(map1, map2), |s| {
        let mut map1 = get_map(s.get("map1"))?;
        let map2 = get_map(s.get("map2"))?;
        for (key, value) in map2 {
            map1.insert(key, value);
        }
        Ok(Value::Map(map1))
    });
    def_va!(f, map_remove(map, keys), |s| {
        let mut map = get_map(s.get("map"))?;
        match s.get("keys") {
            Value::List(keys, ..) => for key in keys {
                map.remove(&key);
            },
            key => {
                map.remove(&key);
            }
        }
        Ok(Value::Map(map))
    });
    def!(f, map_keys(map), |s| {
        let map = get_map(s.get("map"))?;
        Ok(Value::List(map.keys(), ListSeparator::Comma, false))
    });
    def!(f, map_values(map), |s| {
        let map = get_map(s.get("map"))?;
        Ok(Value::List(map.values(), ListSeparator::Comma, false))
    });
    def!(f, map_has_key(map, key), |s| {
        let map = get_map(s.get("map"))?;
        Ok(Value::bool(map.contains_key(&s.get("key"))))
    });
}

fn get_map(v: Value) -> Result<OrderMap<Value, Value>, Error> {
    match v {
        Value::Map(m) => Ok(m),
        // An empty map and an empty list looks the same
        Value::List(ref l, ..) if l.is_empty() => Ok(OrderMap::new()),
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
            check_val(
                "map-has-key((\"foo\": 1, \"bar\": 2), \"foo\");",
                "true",
            )
        }
        #[test]
        fn b() {
            check_val(
                "map-has-key((\"foo\": 1, \"bar\": 2), \"baz\");",
                "false",
            )
        }
    }

    fn check_val(src: &str, correct: &str) {
        use variablescope::test::do_evaluate;
        assert_eq!(do_evaluate(&[], src.as_bytes()), correct)
    }
}
