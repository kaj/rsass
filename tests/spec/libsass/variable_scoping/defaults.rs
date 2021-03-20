//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/defaults.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$i: 9;\
            \n$z: 3 !default;\
            \ndiv {\
            \n  asd: $i;\
            \n  $i: 99 !global;\
            \n  $n: 42 !global;\
            \n  qwe: $i;\
            \n  zapf: $z;\
            \n  $z: 84;\
            \n  ding: $z;\
            \n}\
            \ndiv {\
            \n  foo: $n;\
            \n  foo: $i;\
            \n  $i: 999;\
            \n  $n: 999;\
            \n  foo: $n;\
            \n  foo: $i;\
            \n  div {\
            \n    $i: 9999;\
            \n    $n: 9999 !default;\
            \n    bar: $i;\
            \n    bar: $n;\
            \n  }\
            \n  baz: $i;\
            \n}\
            \ndiv {\
            \n  asd: $i;\
            \n  qwe: $n;\
            \n  zap: $z;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  asd: 9;\
        \n  qwe: 99;\
        \n  zapf: 3;\
        \n  ding: 84;\
        \n}\
        \ndiv {\
        \n  foo: 42;\
        \n  foo: 99;\
        \n  foo: 999;\
        \n  foo: 999;\
        \n  baz: 9999;\
        \n}\
        \ndiv div {\
        \n  bar: 9999;\
        \n  bar: 999;\
        \n}\
        \ndiv {\
        \n  asd: 99;\
        \n  qwe: 42;\
        \n  zap: 3;\
        \n}\
        \n"
    );
}
