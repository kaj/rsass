use super::{Error, Module, SassFunction};
use crate::css::Value;
use crate::ordermap::OrderMap;
use crate::value::ListSeparator;

/// Create the `sass:map` standard module.
///
/// Should conform to
/// [the specification](https://sass-lang.com/documentation/modules/map).
pub fn create_module() -> Module {
    let mut f = Module::new();
    // TODO deep_merge and deep_remove
    def_va!(f, get(map, key, keys), |s| {
        let map = get_map(s.get("map")?)?;
        let mut val = map.get(&s.get("key")?).cloned();
        match s.get("keys")? {
            Value::List(keys, ..) => {
                for k in keys {
                    match val {
                        Some(Value::Map(m)) => {
                            val = m.get(&k).cloned();
                        }
                        _ => return Ok(Value::Null),
                    }
                }
            }
            Value::Null => (),
            key => {
                // Single key
                match val {
                    Some(Value::Map(m)) => {
                        val = m.get(&key).cloned();
                    }
                    _ => return Ok(Value::Null),
                }
            } //_ => (),
        };
        Ok(val.unwrap_or(Value::Null))
    });
    def_va!(f, has_key(map, key, keys), |s| {
        let map = get_map(s.get("map")?)?;
        match s.get("keys")? {
            Value::List(keys, ..) => {
                if let Some((last, keys)) = keys.split_last() {
                    let mut val = map.get(&s.get("key")?).cloned();
                    for k in keys {
                        match val {
                            Some(Value::Map(m)) => {
                                val = m.get(&k).cloned();
                            }
                            _ => return Ok(Value::False),
                        }
                    }
                    if let Some(Value::Map(val)) = val {
                        Ok(val.contains_key(last).into())
                    } else {
                        Ok(Value::False)
                    }
                } else {
                    Ok(map.contains_key(&s.get("key")?).into())
                }
            }
            Value::Null => Ok(map.contains_key(&s.get("key")?).into()),
            key => {
                // Single key
                match map.get(&s.get("key")?) {
                    Some(Value::Map(m)) => Ok(m.contains_key(&key).into()),
                    _ => Ok(Value::Null),
                }
            }
        }
    });
    def!(f, keys(map), |s| {
        let map = get_map(s.get("map")?)?;
        Ok(Value::List(map.keys(), ListSeparator::Comma, false))
    });
    // TODO: Merge should be varargs
    def!(f, merge(map1, map2), |s| {
        let mut map1 = get_map(s.get("map1")?)?;
        let map2 = get_map(s.get("map2")?)?;
        for (key, value) in map2 {
            map1.insert(key, value);
        }
        Ok(Value::Map(map1))
    });
    // It's really map_remove(map, keys), but "key" is supported as an
    // alias for "keys", which makes a mess when using more than one
    // positional argument.
    def_va!(f, remove(map, key, keys), |s| {
        let mut map = get_map(s.get("map")?)?;
        let key = s.get("key")?;
        let keys = s.get("keys")?;
        match (key, keys) {
            (first, Value::List(rest, ..)) => {
                map.remove(&first);
                for key in rest {
                    map.remove(&key);
                }
            }
            (Value::List(keys, ..), Value::Null) => {
                for key in keys {
                    map.remove(&key);
                }
            }
            (first, second) => {
                map.remove(&first);
                map.remove(&second);
            }
        }
        Ok(Value::Map(map))
    });
    def!(f, set(map, key, value), |s| {
        // TODO: handle keys... arguments before key for nested maps
        let mut map = get_map(s.get("map")?)?;
        let key = s.get("key")?;
        let value = s.get("value")?;
        map.insert(key, value);
        Ok(Value::Map(map))
    });
    def!(f, values(map), |s| {
        let map = get_map(s.get("map")?)?;
        Ok(Value::List(map.values(), ListSeparator::Comma, false))
    });
    f
}

pub fn expose(m: &Module, global: &mut Module) {
    for &(gname, lname) in &[
        ("map_get", "get"),
        ("map_has_key", "has_key"),
        ("map_keys", "keys"),
        ("map_merge", "merge"),
        ("map_remove", "remove"),
        ("map_values", "values"),
    ] {
        global.insert_function(gname, m.get_function(lname).unwrap().clone());
    }
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
        use crate::variablescope::test::do_evaluate;
        assert_eq!(do_evaluate(&[], src.as_bytes()), correct)
    }
}
