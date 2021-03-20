//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1796.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".parent {\
            \n    .brother, .sister, .cousin {\
            \n        color: green;\
            \n        sel: &;\
            \n\
            \n        $new-sel: ();\
            \n        @each $s in & {\
            \n            $last: nth($s, -1);\
            \n            $new-sel: append($new-sel, $s #{\'+\'} $last, comma);\
            \n            x: $new-sel;\
            \n        }\
            \n        @at-root #{$new-sel} {\
            \n            debug: foo;\
            \n        }\
            \n    }\
            \n}"
        )
        .unwrap(),
        ".parent .brother, .parent .sister, .parent .cousin {\
        \n  color: green;\
        \n  sel: .parent .brother, .parent .sister, .parent .cousin;\
        \n  x: .parent .brother + .brother;\
        \n  x: .parent .brother + .brother, .parent .sister + .sister;\
        \n  x: .parent .brother + .brother, .parent .sister + .sister, .parent .cousin + .cousin;\
        \n}\
        \n.parent .brother + .brother, .parent .sister + .sister, .parent .cousin + .cousin {\
        \n  debug: foo;\
        \n}\
        \n"
    );
}
