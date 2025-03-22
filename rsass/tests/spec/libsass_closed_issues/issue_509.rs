//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_509.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_509")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n$foo: (\
             \n    (key1): (value-1-0),\
             \n    key2: value-2-0,\
             \n    (key6): (value-6-0),\
             \n    key-3-0 key-3-1 key-3-2: value-3-0 value-3-1 value-3-2,\
             \n    key4: (value-4-0, value-4-1, value-4-2),\
             \n    key5: (key-5-0: value-5-1),\
             \n    (key-7-0: key-7-1): (value-7-0: value-7-1),\
             \n    (key-8-0, key-8-1, key-8-2): (value-8-0, value-8-1, value-8-2),\
             \n);\n\
             \ndiv {\
             \n    foo: map.get((foo: 1, bar: 2), foo);\
             \n    foo: map.get((foo: 1, bar: 2), bar);\
             \n    foo: map.get((foo: 1, bar: 2), baz);\
             \n    foo: map.get((), foo);\
             \n    foo: map.get($foo, (key-5-0: value-5-1));\
             \n    foo: map.get($foo, (key2));\
             \n    foo: map.get($foo, (key-3-0 key-3-1 key-3-2));\
             \n}\n"
        ),
        "div {\
         \n  foo: 1;\
         \n  foo: 2;\
         \n  foo: value-2-0;\
         \n  foo: value-3-0 value-3-1 value-3-2;\
         \n}\n"
    );
}
