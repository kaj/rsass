//! Tests auto-converted from "sass-spec/spec/libsass/units/conversion/resolution.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: (0dpi + 1dpi);\
            \n  output: (4.2dpi / 1dpi);\
            \n  output: (4.2dpi * 1dpi / 1dpi);\
            \n  output: (0dpi + 1dpcm);\
            \n  output: (4.2dpi / 1dpcm);\
            \n  output: (4.2dpi * 1dpcm / 1dpi);\
            \n  output: (0dpi + 1dppx);\
            \n  output: (4.2dpi / 1dppx);\
            \n  output: (4.2dpi * 1dppx / 1dpi);\
            \n  output: (0dpcm + 1dpi);\
            \n  output: (4.2dpcm / 1dpi);\
            \n  output: (4.2dpcm * 1dpi / 1dpcm);\
            \n  output: (0dpcm + 1dpcm);\
            \n  output: (4.2dpcm / 1dpcm);\
            \n  output: (4.2dpcm * 1dpcm / 1dpcm);\
            \n  output: (0dpcm + 1dppx);\
            \n  output: (4.2dpcm / 1dppx);\
            \n  output: (4.2dpcm * 1dppx / 1dpcm);\
            \n  output: (0dppx + 1dpi);\
            \n  output: (4.2dppx / 1dpi);\
            \n  output: (4.2dppx * 1dpi / 1dppx);\
            \n  output: (0dppx + 1dpcm);\
            \n  output: (4.2dppx / 1dpcm);\
            \n  output: (4.2dppx * 1dpcm / 1dppx);\
            \n  output: (0dppx + 1dppx);\
            \n  output: (4.2dppx / 1dppx);\
            \n  output: (4.2dppx * 1dppx / 1dppx);\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: 1dpi;\
        \n  output: 4.2;\
        \n  output: 4.2dpi;\
        \n  output: 2.54dpi;\
        \n  output: 1.6535433071;\
        \n  output: 4.2dpcm;\
        \n  output: 96dpi;\
        \n  output: 0.04375;\
        \n  output: 4.2dppx;\
        \n  output: 0.3937007874dpcm;\
        \n  output: 10.668;\
        \n  output: 4.2dpi;\
        \n  output: 1dpcm;\
        \n  output: 4.2;\
        \n  output: 4.2dpcm;\
        \n  output: 37.7952755906dpcm;\
        \n  output: 0.111125;\
        \n  output: 4.2dppx;\
        \n  output: 0.0104166667dppx;\
        \n  output: 403.2;\
        \n  output: 4.2dpi;\
        \n  output: 0.0264583333dppx;\
        \n  output: 158.7401574803;\
        \n  output: 4.2dpcm;\
        \n  output: 1dppx;\
        \n  output: 4.2;\
        \n  output: 4.2dppx;\
        \n}\
        \n"
    );
}
