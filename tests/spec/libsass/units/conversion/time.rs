//! Tests auto-converted from "sass-spec/spec/libsass/units/conversion/time.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: (0s + 1s);\
            \n  output: (4.2s / 1s);\
            \n  output: (4.2s * 1s / 1s);\
            \n  output: (0s + 1ms);\
            \n  output: (4.2s / 1ms);\
            \n  output: (4.2s * 1ms / 1s);\
            \n  output: (0ms + 1s);\
            \n  output: (4.2ms / 1s);\
            \n  output: (4.2ms * 1s / 1ms);\
            \n  output: (0ms + 1ms);\
            \n  output: (4.2ms / 1ms);\
            \n  output: (4.2ms * 1ms / 1ms);\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: 1s;\
        \n  output: 4.2;\
        \n  output: 4.2s;\
        \n  output: 0.001s;\
        \n  output: 4200;\
        \n  output: 4.2ms;\
        \n  output: 1000ms;\
        \n  output: 0.0042;\
        \n  output: 4.2s;\
        \n  output: 1ms;\
        \n  output: 4.2;\
        \n  output: 4.2ms;\
        \n}\
        \n"
    );
}
