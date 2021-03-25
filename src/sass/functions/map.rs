use super::{Error, FunctionMap, Name};
use crate::css::Value;
use crate::ordermap::OrderMap;
use crate::value::ListSeparator;
use crate::Scope;

/// Create the `sass:map` standard module.
///
/// Should conform to
/// [the specification](https://sass-lang.com/documentation/modules/map).
pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:map");

    def!(f, deep_merge(map1, map2), |s| {
        let map1 = get_map(s, name!(map1))?;
        let mut map2 = get_va_map(s, name!(map2))?;
        do_deep_merge(&mut map2, &map1);
        Ok(Value::Map(map2))
    });

    def_va!(f, deep_remove(map, key, keys), |s| {
        let mut map = get_map(s, name!(map))?;
        let key = s.get("key")?;
        let keychain = match s.get("keys")? {
            Value::List(mut keys, ..) => {
                keys.insert(0, key);
                keys
            }
            Value::Null => vec![key],
            single => vec![key, single],
        };
        do_deep_remove(&mut map, &keychain);
        Ok(Value::Map(map))
    });

    def_va!(f, get(map, key, keys), |s| {
        let map = get_map(s, name!(map))?;
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
        let map = get_map(s, name!(map))?;
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
        let map = get_map(s, name!(map))?;
        Ok(Value::List(map.keys(), ListSeparator::Comma, false))
    });
    def_va!(f, merge(map1, map2), |s| {
        let mut map1 = get_map(s, name!(map1))?;
        let map2 = get_va_map(s, name!(map2))?;
        for (key, value) in map2 {
            if let (Some(Value::Map(m1)), Value::Map(ref m2)) =
                (map1.get(&key), &value)
            {
                let mut m1 = m1.clone();
                for (key, value) in m2.clone().iter() {
                    m1.insert(key.clone(), value.clone());
                }
                map1.insert(key, Value::Map(m1));
            } else {
                map1.insert(key, value);
            }
        }
        Ok(Value::Map(map1))
    });
    // It's really map_remove(map, keys), but "key" is supported as an
    // alias for "keys", which makes a mess when using more than one
    // positional argument.
    def_va!(f, remove(map, key = b"null", keys = b"null"), |s| {
        let mut map = get_map(s, name!(map))?;
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
    def_va!(f, set(map, args), set);
    def!(f, values(map), |s| {
        let map = get_map(s, name!(map))?;
        Ok(Value::List(map.values(), ListSeparator::Comma, false))
    });
    f
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        (name!(map_get), name!(get)),
        (name!(map_set), name!(set)),
        (name!(map_has_key), name!(has_key)),
        (name!(map_keys), name!(keys)),
        (name!(map_merge), name!(merge)),
        (name!(map_remove), name!(remove)),
        (name!(map_values), name!(values)),
    ] {
        global.insert(gname.clone(), m.get_function(&lname).unwrap().clone());
    }
}

fn get_map(s: &Scope, name: Name) -> Result<OrderMap<Value, Value>, Error> {
    match s.get(name.as_ref())? {
        Value::Map(m) => Ok(m),
        // An empty map and an empty list looks the same
        Value::List(ref l, ..) if l.is_empty() => Ok(OrderMap::new()),
        v => Err(Error::bad_arg(name, &v, "is not a map")),
    }
}

fn get_va_map(
    s: &Scope,
    name: Name,
) -> Result<OrderMap<Value, Value>, Error> {
    match s.get(name.as_ref())? {
        Value::Map(m) => Ok(m),
        Value::List(ref l, ..) => Ok(map_from_list(&l)),
        v => Err(Error::bad_arg(name, &v, "is not a map")),
    }
}

fn map_from_list(values: &[Value]) -> OrderMap<Value, Value> {
    let mut m = OrderMap::new();
    match values.split_first() {
        Some((first, [Value::List(ref l, ..)])) if l.is_empty() => {
            m.insert(first.clone(), Value::Map(OrderMap::new()));
        }
        Some((first, [other])) => {
            m.insert(first.clone(), other.clone());
        }
        Some((first, rest)) => {
            m.insert(first.clone(), Value::Map(map_from_list(rest)));
        }
        None => (),
    }
    m
}

fn do_deep_merge(
    map1: &mut OrderMap<Value, Value>,
    map2: &OrderMap<Value, Value>,
) {
    for (key, value) in map2.iter() {
        match (map1.get_mut(&key), value) {
            (Some(Value::Map(m1)), Value::Map(ref m2)) => {
                do_deep_merge(m1, m2);
            }
            (Some(v1), v2) if v1.is_null() => {
                map1.insert(key.clone(), v2.clone());
            }
            (None, v2) => {
                map1.insert(key.clone(), v2.clone());
            }
            _ => (),
        }
    }
}

fn do_deep_remove(map: &mut OrderMap<Value, Value>, keys: &[Value]) {
    match keys.len() {
        0 => (), // Error?  Or just fine?
        1 => {
            map.remove(&keys[0]);
        }
        _ => {
            if let Some(Value::Map(inner)) = map.get_mut(&keys[0]) {
                do_deep_remove(inner, &keys[1..]);
            }
        }
    }
}

fn set(s: &Scope) -> Result<Value, Error> {
    let map = get_map(s, name!(map))?;
    let mut args = match s.get("args")? {
        Value::List(v, ..) => v,
        _ => return Err(Error::error("Expected $args to contain a value")),
    };
    if let Some(value) = args.pop() {
        Ok(Value::Map(set_inner(map, &args, value)?))
    } else {
        Err(Error::error("Expected $args to contain a key"))
    }
}
fn set_inner(
    mut map: OrderMap<Value, Value>,
    keys: &[Value],
    value: Value,
) -> Result<OrderMap<Value, Value>, Error> {
    if let Some((key, rest)) = keys.split_first() {
        if rest.is_empty() {
            map.insert(key.clone(), value);
            Ok(map)
        } else {
            let inner = match map.get(&key) {
                Some(Value::Map(inner)) => inner.clone(),
                _ => OrderMap::new(),
            };
            let inner = set_inner(inner, rest, value)?;
            map.insert(key.clone(), Value::Map(inner));
            Ok(map)
        }
    } else {
        Err(Error::error("Expected $args to contain a value"))
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
