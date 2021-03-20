//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1486.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$a: 41px;\
            \n\
            \n@function a() {\
            \n  @return 42px;\
            \n}\
            \n\
            \nfoo {\
            \n  foo: $a -121px;\
            \n  foo: ($a -122px);\
            \n  foo: $a*3-123px;\
            \n  foo: ($a*3-124px);\
            \n  foo: $a*3 -123px;\
            \n  foo: ($a*3 -124px);\
            \n  foo: $a*3 - 123px;\
            \n  foo: ($a*3 - 124px);\
            \n  foo: $a*3- 123px;\
            \n  foo: ($a*3- 124px);\
            \n  foo: $a*3- 123px;\
            \n  foo: ($a*3- 124px);\
            \n}\
            \n\
            \nbar {\
            \n  bar: a() -121px;\
            \n  bar: (a() -122px);\
            \n  bar: a()*3-123px;\
            \n  bar: (a()*3-124px);\
            \n  bar: a()*3 -123px;\
            \n  bar: (a()*3 -124px);\
            \n  bar: a()*3 - 123px;\
            \n  bar: (a()*3 - 124px);\
            \n  bar: a()*3- 123px;\
            \n  bar: (a()*3- 124px);\
            \n  bar: a()*3- 123px;\
            \n  bar: (a()*3- 124px);\
            \n}\
            \n\
            \nbaz {\
            \n  baz: 43px -121px;\
            \n  baz: (43px -122px);\
            \n  baz: 43px*3-123px;\
            \n  baz: (43px*3-124px);\
            \n  baz: 43px*3 -123px;\
            \n  baz: (43px*3 -124px);\
            \n  baz: 43px*3 - 123px;\
            \n  baz: (43px*3 - 124px);\
            \n  baz: 43px*3- 123px;\
            \n  baz: (43px*3- 124px);\
            \n  baz: 43px*3- 123px;\
            \n  baz: (43px*3- 124px);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: 41px -121px;\
        \n  foo: 41px -122px;\
        \n  foo: 0px;\
        \n  foo: -1px;\
        \n  foo: 123px -123px;\
        \n  foo: 123px -124px;\
        \n  foo: 0px;\
        \n  foo: -1px;\
        \n  foo: 0px;\
        \n  foo: -1px;\
        \n  foo: 0px;\
        \n  foo: -1px;\
        \n}\
        \nbar {\
        \n  bar: 42px -121px;\
        \n  bar: 42px -122px;\
        \n  bar: 3px;\
        \n  bar: 2px;\
        \n  bar: 126px -123px;\
        \n  bar: 126px -124px;\
        \n  bar: 3px;\
        \n  bar: 2px;\
        \n  bar: 3px;\
        \n  bar: 2px;\
        \n  bar: 3px;\
        \n  bar: 2px;\
        \n}\
        \nbaz {\
        \n  baz: 43px -121px;\
        \n  baz: 43px -122px;\
        \n  baz: 6px;\
        \n  baz: 5px;\
        \n  baz: 129px -123px;\
        \n  baz: 129px -124px;\
        \n  baz: 6px;\
        \n  baz: 5px;\
        \n  baz: 6px;\
        \n  baz: 5px;\
        \n  baz: 6px;\
        \n  baz: 5px;\
        \n}\
        \n"
    );
}
