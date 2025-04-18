//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/long-selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("long-selector")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "html, body, div, span, iframe, h1, h2, h3, h4, h5, h6, p, blockquote, pre, a, ab, br, address, cite, code, del, dfn, em, img, ins, kbd, q, samp, small, strong, su, b, sup, var, b, u, i, dl, dt, dd, ol, ul, li, fieldset, form, label, legend, tab, le, caption, tbody, tfoot, thead, tr, th, td {\
             \n  border: 0;\
             \n  font-size: 100%;\
             \n  font: inherit;\
             \n  margin: 0;\
             \n  padding: 0;\
             \n  vertical-align: baseline;\
             \n  hey, ho, hoo {\
             \n    blah: bloo;\
             \n    blee: bleh;\
             \n  }\
             \n}\n"
        ),
        "html, body, div, span, iframe, h1, h2, h3, h4, h5, h6, p, blockquote, pre, a, ab, br, address, cite, code, del, dfn, em, img, ins, kbd, q, samp, small, strong, su, b, sup, var, b, u, i, dl, dt, dd, ol, ul, li, fieldset, form, label, legend, tab, le, caption, tbody, tfoot, thead, tr, th, td {\
         \n  border: 0;\
         \n  font-size: 100%;\
         \n  font: inherit;\
         \n  margin: 0;\
         \n  padding: 0;\
         \n  vertical-align: baseline;\
         \n}\
         \nhtml hey, html ho, html hoo, body hey, body ho, body hoo, div hey, div ho, div hoo, span hey, span ho, span hoo, iframe hey, iframe ho, iframe hoo, h1 hey, h1 ho, h1 hoo, h2 hey, h2 ho, h2 hoo, h3 hey, h3 ho, h3 hoo, h4 hey, h4 ho, h4 hoo, h5 hey, h5 ho, h5 hoo, h6 hey, h6 ho, h6 hoo, p hey, p ho, p hoo, blockquote hey, blockquote ho, blockquote hoo, pre hey, pre ho, pre hoo, a hey, a ho, a hoo, ab hey, ab ho, ab hoo, br hey, br ho, br hoo, address hey, address ho, address hoo, cite hey, cite ho, cite hoo, code hey, code ho, code hoo, del hey, del ho, del hoo, dfn hey, dfn ho, dfn hoo, em hey, em ho, em hoo, img hey, img ho, img hoo, ins hey, ins ho, ins hoo, kbd hey, kbd ho, kbd hoo, q hey, q ho, q hoo, samp hey, samp ho, samp hoo, small hey, small ho, small hoo, strong hey, strong ho, strong hoo, su hey, su ho, su hoo, b hey, b ho, b hoo, sup hey, sup ho, sup hoo, var hey, var ho, var hoo, b hey, b ho, b hoo, u hey, u ho, u hoo, i hey, i ho, i hoo, dl hey, dl ho, dl hoo, dt hey, dt ho, dt hoo, dd hey, dd ho, dd hoo, ol hey, ol ho, ol hoo, ul hey, ul ho, ul hoo, li hey, li ho, li hoo, fieldset hey, fieldset ho, fieldset hoo, form hey, form ho, form hoo, label hey, label ho, label hoo, legend hey, legend ho, legend hoo, tab hey, tab ho, tab hoo, le hey, le ho, le hoo, caption hey, caption ho, caption hoo, tbody hey, tbody ho, tbody hoo, tfoot hey, tfoot ho, tfoot hoo, thead hey, thead ho, thead hoo, tr hey, tr ho, tr hoo, th hey, th ho, th hoo, td hey, td ho, td hoo {\
         \n  blah: bloo;\
         \n  blee: bleh;\
         \n}\n"
    );
}
