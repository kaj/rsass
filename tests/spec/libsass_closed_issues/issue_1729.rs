//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1729.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%place-to-go {\
            \n  font-size: 1em;\
            \n}\
            \n\
            \na::foo(1){\
            \n  @extend %place-to-go;\
            \n}\
            \na::foo(2){\
            \n  @extend %place-to-go;\
            \n}\
            \nb::foo(1){\
            \n  @extend %place-to-go;\
            \n}\
            \nb::foo(2){\
            \n  @extend %place-to-go;\
            \n}\
            \n\
            \n:bar(1){\
            \n  @extend %place-to-go;\
            \n}\
            \n:bar(2){\
            \n  @extend %place-to-go;\
            \n}\
            \n:bar(3){\
            \n  @extend %place-to-go;\
            \n}\
            \n\
            \n[foo]{\
            \n  @extend %place-to-go;\
            \n}\
            \n[bar]{\
            \n  @extend %place-to-go;\
            \n}\
            \n[baz]{\
            \n  @extend %place-to-go;\
            \n}\
            \n\
            \n[bar=\"1\"]{\
            \n  @extend %place-to-go;\
            \n}\
            \n[bar=\"2\"]{\
            \n  @extend %place-to-go;\
            \n}\
            \n[bar=\"3\"]{\
            \n  @extend %place-to-go;\
            \n}\
            \n"
        )
        .unwrap(),
        "[bar=\"3\"], [bar=\"2\"], [bar=\"1\"], [baz], [bar], [foo], :bar(3), :bar(2), :bar(1), b::foo(2), b::foo(1), a::foo(2), a::foo(1) {\
        \n  font-size: 1em;\
        \n}\
        \n"
    );
}
