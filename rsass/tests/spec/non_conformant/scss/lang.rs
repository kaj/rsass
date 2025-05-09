//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/lang.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lang")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "h1:lang(as),h1:lang(bn),h1:lang(gu),h1:lang(hi),h1:lang(kn),h1:lang(ml),h1:lang(mr),h1:lang(or),h1:lang(pa),h1:lang(sa),h1:lang(ta),h1:lang(te) {\
             \n  line-height:1.5em !important\
             \n}\
             \nh2:lang(as),h3:lang(as),h4:lang(as),h5:lang(as),h6:lang(as),h2:lang(bn),h3:lang(bn),h4:lang(bn),h5:lang(bn),h6:lang(bn),h2:lang(gu),h3:lang(gu),h4:lang(gu),h5:lang(gu),h6:lang(gu),h2:lang(hi),h3:lang(hi),h4:lang(hi),h5:lang(hi),h6:lang(hi),h2:lang(kn),h3:lang(kn),h4:lang(kn),h5:lang(kn),h6:lang(kn),h2:lang(ml),h3:lang(ml),h4:lang(ml),h5:lang(ml),h6:lang(ml),h2:lang(mr),h3:lang(mr),h4:lang(mr),h5:lang(mr),h6:lang(mr),h2:lang(or),h3:lang(or),h4:lang(or),h5:lang(or),h6:lang(or),h2:lang(pa),h3:lang(pa),h4:lang(pa),h5:lang(pa),h6:lang(pa),h2:lang(sa),h3:lang(sa),h4:lang(sa),h5:lang(sa),h6:lang(sa),h2:lang(ta),h3:lang(ta),h4:lang(ta),h5:lang(ta),h6:lang(ta),h2:lang(te),h3:lang(te),h4:lang(te),h5:lang(te),h6:lang(te)\
             \n{\
             \n  line-height:1.2em\
             \n}\
             \nol:lang(bcc) li,ol:lang(bqi) li,ol:lang(fa) li,ol:lang(glk) li,ol:lang(kk-arab) li,ol:lang(mzn) li {\
             \n  list-style-type:-moz-persian;list-style-type:persian\
             \n}\
             \nol:lang(ckb) li {\
             \n  list-style-type:-moz-arabic-indic;list-style-type:arabic-indic\
             \n}\
             \nol:lang(as) li,ol:lang(bn) li{\
             \n  list-style-type:-moz-bengali;list-style-type:bengali\
             \n}\
             \nol:lang(or) li {\
             \n  list-style-type:-moz-oriya;list-style-type:oriya\
             \n}"
        ),
        "h1:lang(as), h1:lang(bn), h1:lang(gu), h1:lang(hi), h1:lang(kn), h1:lang(ml), h1:lang(mr), h1:lang(or), h1:lang(pa), h1:lang(sa), h1:lang(ta), h1:lang(te) {\
         \n  line-height: 1.5em !important;\
         \n}\
         \nh2:lang(as), h3:lang(as), h4:lang(as), h5:lang(as), h6:lang(as), h2:lang(bn), h3:lang(bn), h4:lang(bn), h5:lang(bn), h6:lang(bn), h2:lang(gu), h3:lang(gu), h4:lang(gu), h5:lang(gu), h6:lang(gu), h2:lang(hi), h3:lang(hi), h4:lang(hi), h5:lang(hi), h6:lang(hi), h2:lang(kn), h3:lang(kn), h4:lang(kn), h5:lang(kn), h6:lang(kn), h2:lang(ml), h3:lang(ml), h4:lang(ml), h5:lang(ml), h6:lang(ml), h2:lang(mr), h3:lang(mr), h4:lang(mr), h5:lang(mr), h6:lang(mr), h2:lang(or), h3:lang(or), h4:lang(or), h5:lang(or), h6:lang(or), h2:lang(pa), h3:lang(pa), h4:lang(pa), h5:lang(pa), h6:lang(pa), h2:lang(sa), h3:lang(sa), h4:lang(sa), h5:lang(sa), h6:lang(sa), h2:lang(ta), h3:lang(ta), h4:lang(ta), h5:lang(ta), h6:lang(ta), h2:lang(te), h3:lang(te), h4:lang(te), h5:lang(te), h6:lang(te) {\
         \n  line-height: 1.2em;\
         \n}\
         \nol:lang(bcc) li, ol:lang(bqi) li, ol:lang(fa) li, ol:lang(glk) li, ol:lang(kk-arab) li, ol:lang(mzn) li {\
         \n  list-style-type: -moz-persian;\
         \n  list-style-type: persian;\
         \n}\
         \nol:lang(ckb) li {\
         \n  list-style-type: -moz-arabic-indic;\
         \n  list-style-type: arabic-indic;\
         \n}\
         \nol:lang(as) li, ol:lang(bn) li {\
         \n  list-style-type: -moz-bengali;\
         \n  list-style-type: bengali;\
         \n}\
         \nol:lang(or) li {\
         \n  list-style-type: -moz-oriya;\
         \n  list-style-type: oriya;\
         \n}\n"
    );
}
