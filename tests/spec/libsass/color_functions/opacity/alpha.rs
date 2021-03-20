//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/opacity/alpha.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  c0: opacity(rgba(0,0,0,0.0));\
            \n  c1: opacity(rgba(0,0,0,0.1));\
            \n  c2: opacity(rgba(0,0,0,0.2));\
            \n  c3: opacity(rgba(0,0,0,0.3));\
            \n  c4: opacity(rgba(0,0,0,0.4));\
            \n  c5: opacity(rgba(0,0,0,0.5));\
            \n  c6: opacity(rgba(0,0,0,0.6));\
            \n  c7: opacity(rgba(0,0,0,0.7));\
            \n  c8: opacity(rgba(0,0,0,0.8));\
            \n  c9: opacity(rgba(0,0,0,0.9));\
            \n  c10: opacity(rgba(0,0,0,1));\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  c0: 0;\
        \n  c1: 0.1;\
        \n  c2: 0.2;\
        \n  c3: 0.3;\
        \n  c4: 0.4;\
        \n  c5: 0.5;\
        \n  c6: 0.6;\
        \n  c7: 0.7;\
        \n  c8: 0.8;\
        \n  c9: 0.9;\
        \n  c10: 1;\
        \n}\
        \n"
    );
}
