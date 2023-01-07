use super::{is_not, CallError, CheckedArg, FunctionMap, ResolvedArgs};
use crate::css::{Value, ValueMap};
use crate::value::ListSeparator;
use crate::Scope;

/// Create the `sass:map` standard module.
///
/// Should conform to
/// [the specification](https://sass-lang.com/documentation/modules/map).
pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:map");
    let mut g = Scope::new_global(Default::default()); // anonymous

    def!(f, deep_merge(map1, map2), |s| {
        let mut map1 = s.get(name!(map1))?;
        let map2 = s.get_map(name!(map2), as_va_map)?;
        do_deep_merge(&mut map1, map2);
        Ok(Value::Map(map1))
    });

    def_va!(f, deep_remove(map, key, keys), |s| {
        let mut map = s.get(name!(map))?;
        let key = s.get(name!(key))?;
        let keychain = match s.get(name!(keys))? {
            Value::ArgList(mut args) => {
                args.positional.insert(0, key);
                args.positional
            }
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

    // Common to get and has_key
    fn find_value<'a>(
        map: &'a ValueMap,
        key: &Value,
        keys: &Value,
    ) -> Result<Option<&'a Value>, CallError> {
        let mut val = map.get(key);
        match keys {
            Value::ArgList(args) => {
                args.check_no_named().map_err(CallError::msg)?;
                for k in &args.positional {
                    match val {
                        Some(Value::Map(m)) => {
                            val = m.get(k);
                        }
                        _ => return Ok(None),
                    }
                }
            }
            Value::List(keys, ..) => {
                for k in keys {
                    match val {
                        Some(Value::Map(m)) => {
                            val = m.get(k);
                        }
                        _ => return Ok(None),
                    }
                }
            }
            Value::Null => (),
            single_key => match val {
                Some(Value::Map(m)) => {
                    val = m.get(single_key);
                }
                _ => return Ok(None),
            },
        };
        Ok(val)
    }
    def_va!(f, get(map, key, keys), |s| {
        let map = s.get(name!(map))?;
        Ok(find_value(&map, &s.get(name!(key))?, &s.get(name!(keys))?)?
            .cloned()
            .unwrap_or(Value::Null))
    });
    def_va!(f, has_key(map, key, keys), |s| {
        let map = s.get(name!(map))?;
        Ok(find_value(&map, &s.get(name!(key))?, &s.get(name!(keys))?)?
            .is_some()
            .into())
    });
    def!(f, keys(map), |s| {
        let map: ValueMap = s.get(name!(map))?;
        Ok(Value::List(
            map.keys().cloned().collect(),
            Some(ListSeparator::Comma),
            false,
        ))
    });
    def_va!(g, merge(map1, args), |s| {
        let mut map1 = s.get(name!(map1))?;
        let (keys, map2) = match s.get(name!(args))? {
            Value::ArgList(mut args) => {
                if let Some(map2) = args.only_named(&name!(map2)) {
                    (vec![], as_va_map(map2).named(name!(map2))?)
                } else {
                    let mut values = args.positional;
                    let map2 = values
                        .pop()
                        .ok_or_else(|| {
                            CallError::msg("Expected $args to contain a key.")
                        })?
                        .try_into()
                        .named(name!(map2))?;
                    (values, map2)
                }
            }
            direct => (vec![], direct.try_into().named(name!(map2))?),
        };
        fn do_merge(
            mut keys: impl Iterator<Item = Value>,
            map1: &mut ValueMap,
            map2: ValueMap,
        ) {
            if let Some(key) = keys.next() {
                if let Some(Value::Map(m1)) = map1.get_mut(&key) {
                    do_merge(keys, m1, map2);
                } else {
                    let mut m2 = ValueMap::new();
                    do_merge(keys, &mut m2, map2);
                    map1.insert(key, Value::Map(m2));
                }
            } else {
                for (key, value) in map2 {
                    map1.insert(key, value);
                }
            }
        }
        do_merge(keys.into_iter(), &mut map1, map2);
        Ok(Value::Map(map1))
    });
    def_va!(g, remove(map, keys), |s| {
        let mut map: ValueMap = s.get(name!(map))?;
        match s.get(name!(keys))? {
            Value::ArgList(mut args) => {
                if let Some(key) = args.named.remove(&name!(key)) {
                    if args.positional.is_empty() {
                        map.remove(&key);
                    } else {
                        return Err(CallError::msg(
                            "Argument $key was passed both by position and by name."
                        ));
                    }
                }
                args.check_no_named().map_err(CallError::msg)?;
                for key in args.positional {
                    map.remove(&key);
                }
            }
            Value::List(keys, ..) => {
                for key in keys {
                    map.remove(&key);
                }
            }
            key => {
                map.remove(&key);
            }
        }
        Ok(Value::Map(map))
    });
    def_va!(g, set(map, args), set);
    def!(f, values(map), |s| {
        let map: ValueMap = s.get(name!(map))?;
        Ok(Value::List(
            map.values().cloned().collect(),
            Some(ListSeparator::Comma),
            false,
        ))
    });
    f.expose_star(&g);
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
        global.insert(gname.clone(), m.get_lfunction(lname));
    }
}

impl TryFrom<Value> for ValueMap {
    type Error = String;
    fn try_from(v: Value) -> Result<Self, String> {
        match v {
            Value::Map(m) => Ok(m),
            // An empty map and an empty list looks the same
            Value::List(ref l, ..) if l.is_empty() => Ok(ValueMap::new()),
            v => Err(is_not(&v, "a map")),
        }
    }
}

fn as_va_map(v: Value) -> Result<ValueMap, String> {
    match v {
        Value::ArgList(args) => {
            args.check_no_named().map_err(|e| e.to_string())?;
            let mut values = args.positional;
            let mut result = if let Some(last) = values.pop() {
                last.try_into()?
            } else {
                return Err("arglist unexpectedly empty".into());
            };
            while let Some(prev) = values.pop() {
                result = ValueMap::singleton(prev, Value::Map(result));
            }
            Ok(result)
        }
        Value::List(mut values, ..) => {
            let mut result = if let Some(last) = values.pop() {
                last.try_into()?
            } else {
                ValueMap::new()
            };
            while let Some(prev) = values.pop() {
                result = ValueMap::singleton(prev, Value::Map(result));
            }
            Ok(result)
        }
        v => v.try_into(),
    }
}

fn do_deep_merge(map1: &mut ValueMap, map2: ValueMap) {
    for (key, value) in map2 {
        match (map1.get_mut(&key), value) {
            (Some(Value::Map(ref mut m1)), Value::Map(m2)) => {
                do_deep_merge(m1, m2);
            }
            (Some(Value::Map(_)), Value::List(ref l, ..)) if l.is_empty() => {
                // nop; empty list is same thing as empty map
            }
            (_, v2) => {
                map1.insert(key, v2);
            }
        }
    }
}

fn do_deep_remove(map: &mut ValueMap, keys: &[Value]) {
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

fn set(s: &ResolvedArgs) -> Result<Value, CallError> {
    let map = s.get(name!(map))?;
    match s.get(name!(args))? {
        Value::ArgList(mut args) => {
            let keys = match args.named.remove(&"keys".into()) {
                Some(Value::List(v, ..)) => Some(v),
                Some(v) => Some(vec![v]),
                None => None,
            };
            let key = args.named.remove(&"key".into());
            if key.is_none() && keys.is_none() && args.positional.is_empty() {
                return Err(CallError::msg(
                    "Expected $args to contain a key.",
                ));
            }
            let value = args
                .named
                .remove(&"value".into())
                .or_else(|| {
                    if key.is_some()
                        || keys.is_some()
                        || args.positional.len() > 1
                    {
                        args.positional.pop()
                    } else {
                        None
                    }
                })
                .ok_or_else(|| {
                    CallError::msg("Expected $args to contain a value.")
                })?;

            let mut keys = match (keys, args.positional.is_empty()) {
                (Some(keys), true) => keys,
                (None, _) => args.positional,
                (Some(_), false) => {
                    return Err(CallError::msg(
                        "Got $keys both by name and by position.",
                    ))
                }
            };
            if let Some(key) = key {
                keys.push(key);
            }
            Ok(Value::Map(set_inner(map, &keys, value)?))
        }
        Value::List(mut v, ..) => {
            if let Some(value) = v.pop() {
                Ok(Value::Map(set_inner(map, &v, value)?))
            } else {
                Err(CallError::msg("Expected $args to contain a key."))
            }
        }
        Value::Map(mut args) => {
            let mut keys = match args.remove(&"keys".into()) {
                Some(Value::List(v, ..)) => v,
                Some(v) => vec![v],
                None => vec![],
            };
            if let Some(key) = args.remove(&"key".into()) {
                keys.push(key);
            }
            let value = args.remove(&"value".into()).ok_or_else(|| {
                CallError::msg("Expected $args to contain a value.")
            })?;
            Ok(Value::Map(set_inner(map, &keys, value)?))
        }
        _ => Err(CallError::msg("Expected $args to contain a value.")),
    }
}
fn set_inner(
    mut map: ValueMap,
    keys: &[Value],
    value: Value,
) -> Result<ValueMap, CallError> {
    if let Some((key, rest)) = keys.split_first() {
        let value = if rest.is_empty() {
            value
        } else {
            let inner = match map.remove(key) {
                Some(Value::Map(inner)) => inner,
                _ => ValueMap::new(),
            };
            Value::Map(set_inner(inner, rest, value)?)
        };
        map.insert(key.clone(), value);
        Ok(map)
    } else {
        Err(CallError::msg("Expected $args to contain a value."))
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
