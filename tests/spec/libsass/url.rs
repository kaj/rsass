//! Tests auto-converted from "sass-spec/spec/libsass/url.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: pop;\
            \n$y: 123;\
            \n\
            \n\
            \n\
            \ndiv {\
            \n  foo: url(bloo/blah.css);\
            \n  bar: url(http://foo/bar/hux.css);\
            \n  foo: url(fudge#{$x}.css);\
            \n  bar: url(\"http://fudge#{$x}/styles.css\");\
            \n  hux: url(http://box_#{$y}////fudge#{$x}.css);\
            \n  @each $i in (1 2 3 4 5) {\
            \n    hux: url(http://box_#{$y}////fudge#{$x}.css);\
            \n    foo: url(http://blah.com/bar-#{$i}.css);\
            \n  }\
            \n  gloo: url(\"hey#{1+2}.css\");\
            \n  floo: url(hadoop-#{$y+321}.css);\
            \n  flum: image-url(\"fudge.png\", hux);\
            \n  /*****/\
            \n  $bg: \"image.png\";\
            \n  background: url(\"#{$bg}\");\
            \n  //gudge: url(type-of(hello) + length(a b c));\
            \n  mudge: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAACXBIWXMAAAsTAAALEwEAmpwYAAAABGdBTUEAALGOfPtRkwAAACBjSFJNAAB6JQAAgIMAAPn/AACA6QAAdTAAAOpgAAA6mAAAF2+SX8VGAAACeElEQVR42nySy29McRTHP/fOnTvT6bQNrdHKMGGhFkTSELGxwoJY8Q9YWFhYEUJsRSKCsJWikjYSJBIbinpVPJLSRlEkKK2WTnXmvl+/Y4F4tPVJPqtvzjcnJ0cTEQxdY/miFH6gcAJpaWrQl86t05rR9axSKD8UZ6KqJscm5bMdyDDgAYgIBoCORm2G1u0b6w8unJ/bmDG1QtpUmIYiZ8Zk0zEpYmW76tujV9J3/Ep04v0XdR2IDYAdWxYt27Sa8/l8btWIlaYSupgqpNaMUYbC0DUa8qKXWpLGNSvZEETpZO/Z4B5gGQCRMio1xdVfioUIa3AQJ/ZARWhJgkQJKq3wfJ3RwETGhRtPgx7ABtBEhCVNBqViU2tn5+5bLfXmgurIYwJrGFEJmqZh2T4jo2X0YIreZ+7dfeejrcCEiKADfCon3O4fHzp25Nx+8nnqF65lXnEphQUtNBYKaKkMcRgxVY29093JUWCCn+gAORMaTLh0dbCjo/1KO3X1kC6BGIR+QLVioSc+F+9HnW/G1DX+QAcw0j8c/QaHj3UfeN0/MMicEmSL+J5P6DkMDUcfLvZGJ4FwWoHl/lAEXo344zv3dO3ynXJIpg7XdnBtj46bwSnblwH+QQdQ8lsNeNg32nOm/fIh3CGS0OXOQHCv90XYwUyICM2NNX85f26WUnOu5smFzX0vu9qktZjeNtusAbB+XdvfAWDZnjeurX2XST1Y8X6s7zmzYABUrHBaYNshYRC4k340FcZU/1vg2JVpgeP4uJXypHK8soD134In/W+mb+AJvffvvC022It/ve1MaCJCXU6f4UCQy1CbNVONH7/Gw7Md8fsAtddMUh5fveYAAAAASUVORK5CYII=);\
            \n  nudge: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAAAAAA6mKC9AAAAGElEQVQYV2N4DwX/oYBhgARgDJjEAAkAAEC99wFuu0VFAAAAAElFTkSuQmCC);\
            \n  pudge: url(http://wiki.jbussdieker.name/skins/common/images/Checker-16x16.png?2012-05-02T13:40:00Z);\
            \n}\
            \n\
            \np:after {\
            \n  content:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAACXBIWXMAAAsTAAALEwEAmpwYAAAABGdBTUEAALGOfPtRkwAAACBjSFJNAAB6JQAAgIMAAPn/AACA6QAAdTAAAOpgAAA6mAAAF2+SX8VGAAACeElEQVR42nySy29McRTHP/fOnTvT6bQNrdHKMGGhFkTSELGxwoJY8Q9YWFhYEUJsRSKCsJWikjYSJBIbinpVPJLSRlEkKK2WTnXmvl+/Y4F4tPVJPqtvzjcnJ0cTEQxdY/miFH6gcAJpaWrQl86t05rR9axSKD8UZ6KqJscm5bMdyDDgAYgIBoCORm2G1u0b6w8unJ/bmDG1QtpUmIYiZ8Zk0zEpYmW76tujV9J3/Ep04v0XdR2IDYAdWxYt27Sa8/l8btWIlaYSupgqpNaMUYbC0DUa8qKXWpLGNSvZEETpZO/Z4B5gGQCRMio1xdVfioUIa3AQJ/ZARWhJgkQJKq3wfJ3RwETGhRtPgx7ABtBEhCVNBqViU2tn5+5bLfXmgurIYwJrGFEJmqZh2T4jo2X0YIreZ+7dfeejrcCEiKADfCon3O4fHzp25Nx+8nnqF65lXnEphQUtNBYKaKkMcRgxVY29093JUWCCn+gAORMaTLh0dbCjo/1KO3X1kC6BGIR+QLVioSc+F+9HnW/G1DX+QAcw0j8c/QaHj3UfeN0/MMicEmSL+J5P6DkMDUcfLvZGJ4FwWoHl/lAEXo344zv3dO3ynXJIpg7XdnBtj46bwSnblwH+QQdQ8lsNeNg32nOm/fIh3CGS0OXOQHCv90XYwUyICM2NNX85f26WUnOu5smFzX0vu9qktZjeNtusAbB+XdvfAWDZnjeurX2XST1Y8X6s7zmzYABUrHBaYNshYRC4k340FcZU/1vg2JVpgeP4uJXypHK8soD134In/W+mb+AJvffvvC022It/ve1MaCJCXU6f4UCQy1CbNVONH7/Gw7Md8fsAtddMUh5fveYAAAAASUVORK5CYII= );\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: url(bloo/blah.css);\
        \n  bar: url(http://foo/bar/hux.css);\
        \n  foo: url(fudgepop.css);\
        \n  bar: url(\"http://fudgepop/styles.css\");\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-1.css);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-2.css);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-3.css);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-4.css);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-5.css);\
        \n  gloo: url(\"hey3.css\");\
        \n  floo: url(hadoop-444.css);\
        \n  flum: image-url(\"fudge.png\", hux);\
        \n  /*****/\
        \n  background: url(\"image.png\");\
        \n  mudge: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAACXBIWXMAAAsTAAALEwEAmpwYAAAABGdBTUEAALGOfPtRkwAAACBjSFJNAAB6JQAAgIMAAPn/AACA6QAAdTAAAOpgAAA6mAAAF2+SX8VGAAACeElEQVR42nySy29McRTHP/fOnTvT6bQNrdHKMGGhFkTSELGxwoJY8Q9YWFhYEUJsRSKCsJWikjYSJBIbinpVPJLSRlEkKK2WTnXmvl+/Y4F4tPVJPqtvzjcnJ0cTEQxdY/miFH6gcAJpaWrQl86t05rR9axSKD8UZ6KqJscm5bMdyDDgAYgIBoCORm2G1u0b6w8unJ/bmDG1QtpUmIYiZ8Zk0zEpYmW76tujV9J3/Ep04v0XdR2IDYAdWxYt27Sa8/l8btWIlaYSupgqpNaMUYbC0DUa8qKXWpLGNSvZEETpZO/Z4B5gGQCRMio1xdVfioUIa3AQJ/ZARWhJgkQJKq3wfJ3RwETGhRtPgx7ABtBEhCVNBqViU2tn5+5bLfXmgurIYwJrGFEJmqZh2T4jo2X0YIreZ+7dfeejrcCEiKADfCon3O4fHzp25Nx+8nnqF65lXnEphQUtNBYKaKkMcRgxVY29093JUWCCn+gAORMaTLh0dbCjo/1KO3X1kC6BGIR+QLVioSc+F+9HnW/G1DX+QAcw0j8c/QaHj3UfeN0/MMicEmSL+J5P6DkMDUcfLvZGJ4FwWoHl/lAEXo344zv3dO3ynXJIpg7XdnBtj46bwSnblwH+QQdQ8lsNeNg32nOm/fIh3CGS0OXOQHCv90XYwUyICM2NNX85f26WUnOu5smFzX0vu9qktZjeNtusAbB+XdvfAWDZnjeurX2XST1Y8X6s7zmzYABUrHBaYNshYRC4k340FcZU/1vg2JVpgeP4uJXypHK8soD134In/W+mb+AJvffvvC022It/ve1MaCJCXU6f4UCQy1CbNVONH7/Gw7Md8fsAtddMUh5fveYAAAAASUVORK5CYII=);\
        \n  nudge: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAAAAAA6mKC9AAAAGElEQVQYV2N4DwX/oYBhgARgDJjEAAkAAEC99wFuu0VFAAAAAElFTkSuQmCC);\
        \n  pudge: url(http://wiki.jbussdieker.name/skins/common/images/Checker-16x16.png?2012-05-02T13:40:00Z);\
        \n}\
        \np:after {\
        \n  content: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAACXBIWXMAAAsTAAALEwEAmpwYAAAABGdBTUEAALGOfPtRkwAAACBjSFJNAAB6JQAAgIMAAPn/AACA6QAAdTAAAOpgAAA6mAAAF2+SX8VGAAACeElEQVR42nySy29McRTHP/fOnTvT6bQNrdHKMGGhFkTSELGxwoJY8Q9YWFhYEUJsRSKCsJWikjYSJBIbinpVPJLSRlEkKK2WTnXmvl+/Y4F4tPVJPqtvzjcnJ0cTEQxdY/miFH6gcAJpaWrQl86t05rR9axSKD8UZ6KqJscm5bMdyDDgAYgIBoCORm2G1u0b6w8unJ/bmDG1QtpUmIYiZ8Zk0zEpYmW76tujV9J3/Ep04v0XdR2IDYAdWxYt27Sa8/l8btWIlaYSupgqpNaMUYbC0DUa8qKXWpLGNSvZEETpZO/Z4B5gGQCRMio1xdVfioUIa3AQJ/ZARWhJgkQJKq3wfJ3RwETGhRtPgx7ABtBEhCVNBqViU2tn5+5bLfXmgurIYwJrGFEJmqZh2T4jo2X0YIreZ+7dfeejrcCEiKADfCon3O4fHzp25Nx+8nnqF65lXnEphQUtNBYKaKkMcRgxVY29093JUWCCn+gAORMaTLh0dbCjo/1KO3X1kC6BGIR+QLVioSc+F+9HnW/G1DX+QAcw0j8c/QaHj3UfeN0/MMicEmSL+J5P6DkMDUcfLvZGJ4FwWoHl/lAEXo344zv3dO3ynXJIpg7XdnBtj46bwSnblwH+QQdQ8lsNeNg32nOm/fIh3CGS0OXOQHCv90XYwUyICM2NNX85f26WUnOu5smFzX0vu9qktZjeNtusAbB+XdvfAWDZnjeurX2XST1Y8X6s7zmzYABUrHBaYNshYRC4k340FcZU/1vg2JVpgeP4uJXypHK8soD134In/W+mb+AJvffvvC022It/ve1MaCJCXU6f4UCQy1CbNVONH7/Gw7Md8fsAtddMUh5fveYAAAAASUVORK5CYII=);\
        \n}\
        \n"
    );
}
