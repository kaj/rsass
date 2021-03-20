//! Tests auto-converted from "sass-spec/spec/libsass/units/conversion/frequency.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: (0Hz + 1Hz);\
            \n  output: (4.2Hz / 1Hz);\
            \n  output: (4.2Hz * 1Hz / 1Hz);\
            \n  output: (0Hz + 1kHz);\
            \n  output: (4.2Hz / 1kHz);\
            \n  output: (4.2Hz * 1kHz / 1Hz);\
            \n  output: (0kHz + 1Hz);\
            \n  output: (4.2kHz / 1Hz);\
            \n  output: (4.2kHz * 1Hz / 1kHz);\
            \n  output: (0kHz + 1kHz);\
            \n  output: (4.2kHz / 1kHz);\
            \n  output: (4.2kHz * 1kHz / 1kHz);\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: 1Hz;\
        \n  output: 4.2;\
        \n  output: 4.2Hz;\
        \n  output: 1000Hz;\
        \n  output: 0.0042;\
        \n  output: 4.2kHz;\
        \n  output: 0.001kHz;\
        \n  output: 4200;\
        \n  output: 4.2Hz;\
        \n  output: 1kHz;\
        \n  output: 4.2;\
        \n  output: 4.2kHz;\
        \n}\
        \n"
    );
}
