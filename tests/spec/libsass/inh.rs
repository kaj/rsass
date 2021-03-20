//! Tests auto-converted from "sass-spec/spec/libsass/inh.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo.a {\
            \n  width: 10px;\
            \n}\
            \n\
            \nbar {\
            \n  color: red;\
            \n  @extend foo;\
            \n}"
        )
        .unwrap(),
        "foo.a, bar.a {\
        \n  width: 10px;\
        \n}\
        \nbar {\
        \n  color: red;\
        \n}\
        \n"
    );
}
