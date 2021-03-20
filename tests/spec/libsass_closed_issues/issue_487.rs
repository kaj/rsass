//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_487.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "\
            \n@mixin flex($grow: 1, $shrink: null, $basis: null) {\
            \n  -webkit-box-flex: $grow;\
            \n  -webkit-flex: $grow $shrink $basis;\
            \n  -moz-box-flex: $grow;\
            \n  -moz-flex: $grow $shrink $basis;\
            \n  -ms-flex: $grow $shrink $basis;\
            \n  flex: $grow $shrink $basis;\
            \n}\
            \n\
            \n[flex] {\
            \n  @include flex;\
            \n}\
            \n"
        )
        .unwrap(),
        "[flex] {\
        \n  -webkit-box-flex: 1;\
        \n  -webkit-flex: 1;\
        \n  -moz-box-flex: 1;\
        \n  -moz-flex: 1;\
        \n  -ms-flex: 1;\
        \n  flex: 1;\
        \n}\
        \n"
    );
}
