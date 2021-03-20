//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_239.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$gutter: 100% / 36.2;\r\
            \n    $gutter_em: 1rem; //This needs to be rem to not mess up margins\r\
            \n\r\
            \n// This calculate the gutter\r\
            \n@function col_width($n, $use_calc: false) {\r\
            \n    $divisor: 12 / $n;\r\
            \n    @if ($use_calc) {\r\
            \n        $gutter_offset: $gutter_em * ($divisor - 1);\r\
            \n        @return calc((100% - #{$gutter_offset}) / #{$divisor});\r\
            \n    }\r\
            \n    @else {\r\
            \n        @return (100% - $gutter * ($divisor - 1)) / $divisor;\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \n// Each number here becomes a grid: onecol, twocol etc. \r\
            \n$grids: one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve;\r\
            \n$i: 1;\r\
            \n@each $grid in $grids {\r\
            \n    .#{$grid}col {\r\
            \n        width: col_width( $i );\r\
            \n        width: col_width( $i, true );\r\
            \n    }\r\
            \n\r\
            \n    %#{$grid}col {\r\
            \n        width: col_width( $i );\r\
            \n        width: col_width( $i, true );\r\
            \n    }\r\
            \n    $i: $i + 1;\r\
            \n}"
        )
        .unwrap(),
        ".onecol {\
        \n  width: 5.8011049724%;\
        \n  width: calc((100% - 11rem) / 12);\
        \n}\
        \n.twocol {\
        \n  width: 14.364640884%;\
        \n  width: calc((100% - 5rem) / 6);\
        \n}\
        \n.threecol {\
        \n  width: 22.9281767956%;\
        \n  width: calc((100% - 3rem) / 4);\
        \n}\
        \n.fourcol {\
        \n  width: 31.4917127072%;\
        \n  width: calc((100% - 2rem) / 3);\
        \n}\
        \n.fivecol {\
        \n  width: 40.0552486188%;\
        \n  width: calc((100% - 1.4rem) / 2.4);\
        \n}\
        \n.sixcol {\
        \n  width: 48.6187845304%;\
        \n  width: calc((100% - 1rem) / 2);\
        \n}\
        \n.sevencol {\
        \n  width: 57.182320442%;\
        \n  width: calc((100% - 0.7142857143rem) / 1.7142857143);\
        \n}\
        \n.eightcol {\
        \n  width: 65.7458563536%;\
        \n  width: calc((100% - 0.5rem) / 1.5);\
        \n}\
        \n.ninecol {\
        \n  width: 74.3093922652%;\
        \n  width: calc((100% - 0.3333333333rem) / 1.3333333333);\
        \n}\
        \n.tencol {\
        \n  width: 82.8729281768%;\
        \n  width: calc((100% - 0.2rem) / 1.2);\
        \n}\
        \n.elevencol {\
        \n  width: 91.4364640884%;\
        \n  width: calc((100% - 0.0909090909rem) / 1.0909090909);\
        \n}\
        \n.twelvecol {\
        \n  width: 100%;\
        \n  width: calc((100% - 0rem) / 1);\
        \n}\
        \n"
    );
}
