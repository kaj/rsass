//! Tests auto-converted from "sass-spec/spec/libsass/units/conversion/size.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: (0px + 1px);\
            \n  output: (4.2px / 1px);\
            \n  output: (4.2px * 1px / 1px);\
            \n  output: (0px + 1pt);\
            \n  output: (4.2px / 1pt);\
            \n  output: (4.2px * 1pt / 1px);\
            \n  output: (0px + 1pc);\
            \n  output: (4.2px / 1pc);\
            \n  output: (4.2px * 1pc / 1px);\
            \n  output: (0px + 1mm);\
            \n  output: (4.2px / 1mm);\
            \n  output: (4.2px * 1mm / 1px);\
            \n  output: (0px + 1cm);\
            \n  output: (4.2px / 1cm);\
            \n  output: (4.2px * 1cm / 1px);\
            \n  output: (0px + 1in);\
            \n  output: (4.2px / 1in);\
            \n  output: (4.2px * 1in / 1px);\
            \n  output: (0pt + 1px);\
            \n  output: (4.2pt / 1px);\
            \n  output: (4.2pt * 1px / 1pt);\
            \n  output: (0pt + 1pt);\
            \n  output: (4.2pt / 1pt);\
            \n  output: (4.2pt * 1pt / 1pt);\
            \n  output: (0pt + 1pc);\
            \n  output: (4.2pt / 1pc);\
            \n  output: (4.2pt * 1pc / 1pt);\
            \n  output: (0pt + 1mm);\
            \n  output: (4.2pt / 1mm);\
            \n  output: (4.2pt * 1mm / 1pt);\
            \n  output: (0pt + 1cm);\
            \n  output: (4.2pt / 1cm);\
            \n  output: (4.2pt * 1cm / 1pt);\
            \n  output: (0pt + 1in);\
            \n  output: (4.2pt / 1in);\
            \n  output: (4.2pt * 1in / 1pt);\
            \n  output: (0pc + 1px);\
            \n  output: (4.2pc / 1px);\
            \n  output: (4.2pc * 1px / 1pc);\
            \n  output: (0pc + 1pt);\
            \n  output: (4.2pc / 1pt);\
            \n  output: (4.2pc * 1pt / 1pc);\
            \n  output: (0pc + 1pc);\
            \n  output: (4.2pc / 1pc);\
            \n  output: (4.2pc * 1pc / 1pc);\
            \n  output: (0pc + 1mm);\
            \n  output: (4.2pc / 1mm);\
            \n  output: (4.2pc * 1mm / 1pc);\
            \n  output: (0pc + 1cm);\
            \n  output: (4.2pc / 1cm);\
            \n  output: (4.2pc * 1cm / 1pc);\
            \n  output: (0pc + 1in);\
            \n  output: (4.2pc / 1in);\
            \n  output: (4.2pc * 1in / 1pc);\
            \n  output: (0mm + 1px);\
            \n  output: (4.2mm / 1px);\
            \n  output: (4.2mm * 1px / 1mm);\
            \n  output: (0mm + 1pt);\
            \n  output: (4.2mm / 1pt);\
            \n  output: (4.2mm * 1pt / 1mm);\
            \n  output: (0mm + 1pc);\
            \n  output: (4.2mm / 1pc);\
            \n  output: (4.2mm * 1pc / 1mm);\
            \n  output: (0mm + 1mm);\
            \n  output: (4.2mm / 1mm);\
            \n  output: (4.2mm * 1mm / 1mm);\
            \n  output: (0mm + 1cm);\
            \n  output: (4.2mm / 1cm);\
            \n  output: (4.2mm * 1cm / 1mm);\
            \n  output: (0mm + 1in);\
            \n  output: (4.2mm / 1in);\
            \n  output: (4.2mm * 1in / 1mm);\
            \n  output: (0cm + 1px);\
            \n  output: (4.2cm / 1px);\
            \n  output: (4.2cm * 1px / 1cm);\
            \n  output: (0cm + 1pt);\
            \n  output: (4.2cm / 1pt);\
            \n  output: (4.2cm * 1pt / 1cm);\
            \n  output: (0cm + 1pc);\
            \n  output: (4.2cm / 1pc);\
            \n  output: (4.2cm * 1pc / 1cm);\
            \n  output: (0cm + 1mm);\
            \n  output: (4.2cm / 1mm);\
            \n  output: (4.2cm * 1mm / 1cm);\
            \n  output: (0cm + 1cm);\
            \n  output: (4.2cm / 1cm);\
            \n  output: (4.2cm * 1cm / 1cm);\
            \n  output: (0cm + 1in);\
            \n  output: (4.2cm / 1in);\
            \n  output: (4.2cm * 1in / 1cm);\
            \n  output: (0in + 1px);\
            \n  output: (4.2in / 1px);\
            \n  output: (4.2in * 1px / 1in);\
            \n  output: (0in + 1pt);\
            \n  output: (4.2in / 1pt);\
            \n  output: (4.2in * 1pt / 1in);\
            \n  output: (0in + 1pc);\
            \n  output: (4.2in / 1pc);\
            \n  output: (4.2in * 1pc / 1in);\
            \n  output: (0in + 1mm);\
            \n  output: (4.2in / 1mm);\
            \n  output: (4.2in * 1mm / 1in);\
            \n  output: (0in + 1cm);\
            \n  output: (4.2in / 1cm);\
            \n  output: (4.2in * 1cm / 1in);\
            \n  output: (0in + 1in);\
            \n  output: (4.2in / 1in);\
            \n  output: (4.2in * 1in / 1in);\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: 1px;\
        \n  output: 4.2;\
        \n  output: 4.2px;\
        \n  output: 1.3333333333px;\
        \n  output: 3.15;\
        \n  output: 4.2pt;\
        \n  output: 16px;\
        \n  output: 0.2625;\
        \n  output: 4.2pc;\
        \n  output: 3.7795275591px;\
        \n  output: 1.11125;\
        \n  output: 4.2mm;\
        \n  output: 37.7952755906px;\
        \n  output: 0.111125;\
        \n  output: 4.2cm;\
        \n  output: 96px;\
        \n  output: 0.04375;\
        \n  output: 4.2in;\
        \n  output: 0.75pt;\
        \n  output: 5.6;\
        \n  output: 4.2px;\
        \n  output: 1pt;\
        \n  output: 4.2;\
        \n  output: 4.2pt;\
        \n  output: 12pt;\
        \n  output: 0.35;\
        \n  output: 4.2pc;\
        \n  output: 2.8346456693pt;\
        \n  output: 1.4816666667;\
        \n  output: 4.2mm;\
        \n  output: 28.3464566929pt;\
        \n  output: 0.1481666667;\
        \n  output: 4.2cm;\
        \n  output: 72pt;\
        \n  output: 0.0583333333;\
        \n  output: 4.2in;\
        \n  output: 0.0625pc;\
        \n  output: 67.2;\
        \n  output: 4.2px;\
        \n  output: 0.0833333333pc;\
        \n  output: 50.4;\
        \n  output: 4.2pt;\
        \n  output: 1pc;\
        \n  output: 4.2;\
        \n  output: 4.2pc;\
        \n  output: 0.2362204724pc;\
        \n  output: 17.78;\
        \n  output: 4.2mm;\
        \n  output: 2.3622047244pc;\
        \n  output: 1.778;\
        \n  output: 4.2cm;\
        \n  output: 6pc;\
        \n  output: 0.7;\
        \n  output: 4.2in;\
        \n  output: 0.2645833333mm;\
        \n  output: 15.874015748;\
        \n  output: 4.2px;\
        \n  output: 0.3527777778mm;\
        \n  output: 11.905511811;\
        \n  output: 4.2pt;\
        \n  output: 4.2333333333mm;\
        \n  output: 0.9921259843;\
        \n  output: 4.2pc;\
        \n  output: 1mm;\
        \n  output: 4.2;\
        \n  output: 4.2mm;\
        \n  output: 10mm;\
        \n  output: 0.42;\
        \n  output: 4.2cm;\
        \n  output: 25.4mm;\
        \n  output: 0.1653543307;\
        \n  output: 4.2in;\
        \n  output: 0.0264583333cm;\
        \n  output: 158.7401574803;\
        \n  output: 4.2px;\
        \n  output: 0.0352777778cm;\
        \n  output: 119.0551181102;\
        \n  output: 4.2pt;\
        \n  output: 0.4233333333cm;\
        \n  output: 9.9212598425;\
        \n  output: 4.2pc;\
        \n  output: 0.1cm;\
        \n  output: 42;\
        \n  output: 4.2mm;\
        \n  output: 1cm;\
        \n  output: 4.2;\
        \n  output: 4.2cm;\
        \n  output: 2.54cm;\
        \n  output: 1.6535433071;\
        \n  output: 4.2in;\
        \n  output: 0.0104166667in;\
        \n  output: 403.2;\
        \n  output: 4.2px;\
        \n  output: 0.0138888889in;\
        \n  output: 302.4;\
        \n  output: 4.2pt;\
        \n  output: 0.1666666667in;\
        \n  output: 25.2;\
        \n  output: 4.2pc;\
        \n  output: 0.0393700787in;\
        \n  output: 106.68;\
        \n  output: 4.2mm;\
        \n  output: 0.3937007874in;\
        \n  output: 10.668;\
        \n  output: 4.2cm;\
        \n  output: 1in;\
        \n  output: 4.2;\
        \n  output: 4.2in;\
        \n}\
        \n"
    );
}
