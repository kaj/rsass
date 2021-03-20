//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2185.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$bar: true;\r\
            \n\r\
            \n@mixin mixin() {\r\
            \n    mix: in;\r\
            \n}\r\
            \n\r\
            \n@mixin include() {\r\
            \n    @content;\r\
            \n}\r\
            \n\r\
            \np {\r\
            \n    @at-root {\r\
            \n        @if $bar {\r\
            \n            #if {\r\
            \n                height: 100px;\r\
            \n            }\r\
            \n        }\r\
            \n        @if (not $bar) {\r\
            \n        } @else if($bar) {\r\
            \n            #elseif {\r\
            \n                width: 100px;\r\
            \n            }\r\
            \n        }\r\
            \n        @if (not $bar) {\r\
            \n        } @else {\r\
            \n            #else {\r\
            \n                width: 100px;\r\
            \n            }\r\
            \n        }\r\
            \n        @for $i from 1 through 1 {\r\
            \n            #for {\r\
            \n                foo: bar#{$i};\r\
            \n            }\r\
            \n        }\r\
            \n        $i: 0;\r\
            \n        @while ($i == 0) {\r\
            \n            $i: $i + 1;\r\
            \n            #while {\r\
            \n                foo: bar#{$i};\r\
            \n            }\r\
            \n        }\r\
            \n        @each $i in (1) {\r\
            \n            #each {\r\
            \n                foo: bar#{$i};\r\
            \n            }\r\
            \n        }\r\
            \n        #inc {\r\
            \n            @include mixin();\r\
            \n            @include include() {\r\
            \n                inc: lude;\r\
            \n            }\r\
            \n        }\r\
            \n        @supports (display: flex) {\r\
            \n            a {display: flex}\r\
            \n        }\r\
            \n        @foo {\r\
            \n           bar: bat;\r\
            \n        }\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        "#if {\
        \n  height: 100px;\
        \n}\
        \n#elseif {\
        \n  width: 100px;\
        \n}\
        \n#else {\
        \n  width: 100px;\
        \n}\
        \n#for {\
        \n  foo: bar1;\
        \n}\
        \n#while {\
        \n  foo: bar1;\
        \n}\
        \n#each {\
        \n  foo: bar1;\
        \n}\
        \n#inc {\
        \n  mix: in;\
        \n  inc: lude;\
        \n}\
        \n@supports (display: flex) {\
        \n  a {\
        \n    display: flex;\
        \n  }\
        \n}\
        \n@foo {\
        \n  bar: bat;\
        \n}\
        \n"
    );
}
