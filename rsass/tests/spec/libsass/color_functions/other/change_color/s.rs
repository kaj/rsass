//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/other/change-color/s.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("s")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \nfoo {\
             \n  // c-1: change-color(red,$saturation:-1%);\
             \n  c0: color.change(red,$saturation:0%);\
             \n  c1: color.change(red,$saturation:1%);\
             \n  c2: color.change(red,$saturation:2%);\
             \n  c3: color.change(red,$saturation:3%);\
             \n  c4: color.change(red,$saturation:4%);\
             \n  c5: color.change(red,$saturation:5%);\
             \n  c6: color.change(red,$saturation:6%);\
             \n  c7: color.change(red,$saturation:7%);\
             \n  c8: color.change(red,$saturation:8%);\
             \n  c9: color.change(red,$saturation:9%);\
             \n  c10: color.change(red,$saturation:10%);\
             \n  c11: color.change(red,$saturation:11%);\
             \n  c12: color.change(red,$saturation:12%);\
             \n  c13: color.change(red,$saturation:13%);\
             \n  c14: color.change(red,$saturation:14%);\
             \n  c15: color.change(red,$saturation:15%);\
             \n  c16: color.change(red,$saturation:16%);\
             \n  c17: color.change(red,$saturation:17%);\
             \n  c18: color.change(red,$saturation:18%);\
             \n  c19: color.change(red,$saturation:19%);\
             \n  c20: color.change(red,$saturation:20%);\
             \n  c21: color.change(red,$saturation:21%);\
             \n  c22: color.change(red,$saturation:22%);\
             \n  c23: color.change(red,$saturation:23%);\
             \n  c24: color.change(red,$saturation:24%);\
             \n  c25: color.change(red,$saturation:25%);\
             \n  c26: color.change(red,$saturation:26%);\
             \n  c27: color.change(red,$saturation:27%);\
             \n  c28: color.change(red,$saturation:28%);\
             \n  c29: color.change(red,$saturation:29%);\
             \n  c30: color.change(red,$saturation:30%);\
             \n  c31: color.change(red,$saturation:31%);\
             \n  c32: color.change(red,$saturation:32%);\
             \n  c33: color.change(red,$saturation:33%);\
             \n  c34: color.change(red,$saturation:34%);\
             \n  c35: color.change(red,$saturation:35%);\
             \n  c36: color.change(red,$saturation:36%);\
             \n  c37: color.change(red,$saturation:37%);\
             \n  c38: color.change(red,$saturation:38%);\
             \n  c39: color.change(red,$saturation:39%);\
             \n  c40: color.change(red,$saturation:40%);\
             \n  c41: color.change(red,$saturation:41%);\
             \n  c42: color.change(red,$saturation:42%);\
             \n  c43: color.change(red,$saturation:43%);\
             \n  c44: color.change(red,$saturation:44%);\
             \n  c45: color.change(red,$saturation:45%);\
             \n  c46: color.change(red,$saturation:46%);\
             \n  c47: color.change(red,$saturation:47%);\
             \n  c48: color.change(red,$saturation:48%);\
             \n  c49: color.change(red,$saturation:49%);\
             \n  c50: color.change(red,$saturation:50%);\
             \n  c51: color.change(red,$saturation:51%);\
             \n  c52: color.change(red,$saturation:52%);\
             \n  c53: color.change(red,$saturation:53%);\
             \n  c54: color.change(red,$saturation:54%);\
             \n  c55: color.change(red,$saturation:55%);\
             \n  c56: color.change(red,$saturation:56%);\
             \n  c57: color.change(red,$saturation:57%);\
             \n  c58: color.change(red,$saturation:58%);\
             \n  c59: color.change(red,$saturation:59%);\
             \n  c60: color.change(red,$saturation:60%);\
             \n  c61: color.change(red,$saturation:61%);\
             \n  c62: color.change(red,$saturation:62%);\
             \n  c63: color.change(red,$saturation:63%);\
             \n  c64: color.change(red,$saturation:64%);\
             \n  c65: color.change(red,$saturation:65%);\
             \n  c66: color.change(red,$saturation:66%);\
             \n  c67: color.change(red,$saturation:67%);\
             \n  c68: color.change(red,$saturation:68%);\
             \n  c69: color.change(red,$saturation:69%);\
             \n  c70: color.change(red,$saturation:70%);\
             \n  c71: color.change(red,$saturation:71%);\
             \n  c72: color.change(red,$saturation:72%);\
             \n  c73: color.change(red,$saturation:73%);\
             \n  c74: color.change(red,$saturation:74%);\
             \n  c75: color.change(red,$saturation:75%);\
             \n  c76: color.change(red,$saturation:76%);\
             \n  c77: color.change(red,$saturation:77%);\
             \n  c78: color.change(red,$saturation:78%);\
             \n  c79: color.change(red,$saturation:79%);\
             \n  c80: color.change(red,$saturation:80%);\
             \n  c81: color.change(red,$saturation:81%);\
             \n  c82: color.change(red,$saturation:82%);\
             \n  c83: color.change(red,$saturation:83%);\
             \n  c84: color.change(red,$saturation:84%);\
             \n  c85: color.change(red,$saturation:85%);\
             \n  c86: color.change(red,$saturation:86%);\
             \n  c87: color.change(red,$saturation:87%);\
             \n  c88: color.change(red,$saturation:88%);\
             \n  c89: color.change(red,$saturation:89%);\
             \n  c90: color.change(red,$saturation:90%);\
             \n  c91: color.change(red,$saturation:91%);\
             \n  c92: color.change(red,$saturation:92%);\
             \n  c93: color.change(red,$saturation:93%);\
             \n  c94: color.change(red,$saturation:94%);\
             \n  c95: color.change(red,$saturation:95%);\
             \n  c96: color.change(red,$saturation:96%);\
             \n  c97: color.change(red,$saturation:97%);\
             \n  c98: color.change(red,$saturation:98%);\
             \n  c99: color.change(red,$saturation:99%);\
             \n  c100: color.change(red,$saturation:100%);\
             \n  // c101: change-color(red,$saturation:101%);\
             \n}\n"),
        "foo {\
         \n  c0: rgb(50%, 50%, 50%);\
         \n  c1: rgb(50.5%, 49.5%, 49.5%);\
         \n  c2: rgb(51%, 49%, 49%);\
         \n  c3: rgb(51.5%, 48.5%, 48.5%);\
         \n  c4: rgb(52%, 48%, 48%);\
         \n  c5: rgb(52.5%, 47.5%, 47.5%);\
         \n  c6: rgb(53%, 47%, 47%);\
         \n  c7: rgb(53.5%, 46.5%, 46.5%);\
         \n  c8: rgb(54%, 46%, 46%);\
         \n  c9: rgb(54.5%, 45.5%, 45.5%);\
         \n  c10: rgb(55%, 45%, 45%);\
         \n  c11: rgb(55.5%, 44.5%, 44.5%);\
         \n  c12: rgb(56%, 44%, 44%);\
         \n  c13: rgb(56.5%, 43.5%, 43.5%);\
         \n  c14: rgb(57%, 43%, 43%);\
         \n  c15: rgb(57.5%, 42.5%, 42.5%);\
         \n  c16: rgb(58%, 42%, 42%);\
         \n  c17: rgb(58.5%, 41.5%, 41.5%);\
         \n  c18: rgb(59%, 41%, 41%);\
         \n  c19: rgb(59.5%, 40.5%, 40.5%);\
         \n  c20: #996666;\
         \n  c21: rgb(60.5%, 39.5%, 39.5%);\
         \n  c22: rgb(61%, 39%, 39%);\
         \n  c23: rgb(61.5%, 38.5%, 38.5%);\
         \n  c24: rgb(62%, 38%, 38%);\
         \n  c25: rgb(62.5%, 37.5%, 37.5%);\
         \n  c26: rgb(63%, 37%, 37%);\
         \n  c27: rgb(63.5%, 36.5%, 36.5%);\
         \n  c28: rgb(64%, 36%, 36%);\
         \n  c29: rgb(64.5%, 35.5%, 35.5%);\
         \n  c30: rgb(65%, 35%, 35%);\
         \n  c31: rgb(65.5%, 34.5%, 34.5%);\
         \n  c32: rgb(66%, 34%, 34%);\
         \n  c33: rgb(66.5%, 33.5%, 33.5%);\
         \n  c34: rgb(67%, 33%, 33%);\
         \n  c35: rgb(67.5%, 32.5%, 32.5%);\
         \n  c36: rgb(68%, 32%, 32%);\
         \n  c37: rgb(68.5%, 31.5%, 31.5%);\
         \n  c38: rgb(69%, 31%, 31%);\
         \n  c39: rgb(69.5%, 30.5%, 30.5%);\
         \n  c40: rgb(70%, 30%, 30%);\
         \n  c41: rgb(70.5%, 29.5%, 29.5%);\
         \n  c42: rgb(71%, 29%, 29%);\
         \n  c43: rgb(71.5%, 28.5%, 28.5%);\
         \n  c44: rgb(72%, 28%, 28%);\
         \n  c45: rgb(72.5%, 27.5%, 27.5%);\
         \n  c46: rgb(73%, 27%, 27%);\
         \n  c47: rgb(73.5%, 26.5%, 26.5%);\
         \n  c48: rgb(74%, 26%, 26%);\
         \n  c49: rgb(74.5%, 25.5%, 25.5%);\
         \n  c50: rgb(75%, 25%, 25%);\
         \n  c51: rgb(75.5%, 24.5%, 24.5%);\
         \n  c52: rgb(76%, 24%, 24%);\
         \n  c53: rgb(76.5%, 23.5%, 23.5%);\
         \n  c54: rgb(77%, 23%, 23%);\
         \n  c55: rgb(77.5%, 22.5%, 22.5%);\
         \n  c56: rgb(78%, 22%, 22%);\
         \n  c57: rgb(78.5%, 21.5%, 21.5%);\
         \n  c58: rgb(79%, 21%, 21%);\
         \n  c59: rgb(79.5%, 20.5%, 20.5%);\
         \n  c60: #cc3333;\
         \n  c61: rgb(80.5%, 19.5%, 19.5%);\
         \n  c62: rgb(81%, 19%, 19%);\
         \n  c63: rgb(81.5%, 18.5%, 18.5%);\
         \n  c64: rgb(82%, 18%, 18%);\
         \n  c65: rgb(82.5%, 17.5%, 17.5%);\
         \n  c66: rgb(83%, 17%, 17%);\
         \n  c67: rgb(83.5%, 16.5%, 16.5%);\
         \n  c68: rgb(84%, 16%, 16%);\
         \n  c69: rgb(84.5%, 15.5%, 15.5%);\
         \n  c70: rgb(85%, 15%, 15%);\
         \n  c71: rgb(85.5%, 14.5%, 14.5%);\
         \n  c72: rgb(86%, 14%, 14%);\
         \n  c73: rgb(86.5%, 13.5%, 13.5%);\
         \n  c74: rgb(87%, 13%, 13%);\
         \n  c75: rgb(87.5%, 12.5%, 12.5%);\
         \n  c76: rgb(88%, 12%, 12%);\
         \n  c77: rgb(88.5%, 11.5%, 11.5%);\
         \n  c78: rgb(89%, 11%, 11%);\
         \n  c79: rgb(89.5%, 10.5%, 10.5%);\
         \n  c80: rgb(90%, 10%, 10%);\
         \n  c81: rgb(90.5%, 9.5%, 9.5%);\
         \n  c82: rgb(91%, 9%, 9%);\
         \n  c83: rgb(91.5%, 8.5%, 8.5%);\
         \n  c84: rgb(92%, 8%, 8%);\
         \n  c85: rgb(92.5%, 7.5%, 7.5%);\
         \n  c86: rgb(93%, 7%, 7%);\
         \n  c87: rgb(93.5%, 6.5%, 6.5%);\
         \n  c88: rgb(94%, 6%, 6%);\
         \n  c89: rgb(94.5%, 5.5%, 5.5%);\
         \n  c90: rgb(95%, 5%, 5%);\
         \n  c91: rgb(95.5%, 4.5%, 4.5%);\
         \n  c92: rgb(96%, 4%, 4%);\
         \n  c93: rgb(96.5%, 3.5%, 3.5%);\
         \n  c94: rgb(97%, 3%, 3%);\
         \n  c95: rgb(97.5%, 2.5%, 2.5%);\
         \n  c96: rgb(98%, 2%, 2%);\
         \n  c97: rgb(98.5%, 1.5%, 1.5%);\
         \n  c98: rgb(99%, 1%, 1%);\
         \n  c99: rgb(99.5%, 0.5%, 0.5%);\
         \n  c100: red;\
         \n}\n"
    );
}
