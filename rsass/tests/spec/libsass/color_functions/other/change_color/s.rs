//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/other/change-color/s.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("s")
}

#[test]
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
         \n  c0: rgb(127.5, 127.5, 127.5);\
         \n  c1: rgb(128.775, 126.225, 126.225);\
         \n  c2: rgb(130.05, 124.95, 124.95);\
         \n  c3: rgb(131.325, 123.675, 123.675);\
         \n  c4: rgb(132.6, 122.4, 122.4);\
         \n  c5: rgb(133.875, 121.125, 121.125);\
         \n  c6: rgb(135.15, 119.85, 119.85);\
         \n  c7: rgb(136.425, 118.575, 118.575);\
         \n  c8: rgb(137.7, 117.3, 117.3);\
         \n  c9: rgb(138.975, 116.025, 116.025);\
         \n  c10: rgb(140.25, 114.75, 114.75);\
         \n  c11: rgb(141.525, 113.475, 113.475);\
         \n  c12: rgb(142.8, 112.2, 112.2);\
         \n  c13: rgb(144.075, 110.925, 110.925);\
         \n  c14: rgb(145.35, 109.65, 109.65);\
         \n  c15: rgb(146.625, 108.375, 108.375);\
         \n  c16: rgb(147.9, 107.1, 107.1);\
         \n  c17: rgb(149.175, 105.825, 105.825);\
         \n  c18: rgb(150.45, 104.55, 104.55);\
         \n  c19: rgb(151.725, 103.275, 103.275);\
         \n  c20: #996666;\
         \n  c21: rgb(154.275, 100.725, 100.725);\
         \n  c22: rgb(155.55, 99.45, 99.45);\
         \n  c23: rgb(156.825, 98.175, 98.175);\
         \n  c24: rgb(158.1, 96.9, 96.9);\
         \n  c25: rgb(159.375, 95.625, 95.625);\
         \n  c26: rgb(160.65, 94.35, 94.35);\
         \n  c27: rgb(161.925, 93.075, 93.075);\
         \n  c28: rgb(163.2, 91.8, 91.8);\
         \n  c29: rgb(164.475, 90.525, 90.525);\
         \n  c30: rgb(165.75, 89.25, 89.25);\
         \n  c31: rgb(167.025, 87.975, 87.975);\
         \n  c32: rgb(168.3, 86.7, 86.7);\
         \n  c33: rgb(169.575, 85.425, 85.425);\
         \n  c34: rgb(170.85, 84.15, 84.15);\
         \n  c35: rgb(172.125, 82.875, 82.875);\
         \n  c36: rgb(173.4, 81.6, 81.6);\
         \n  c37: rgb(174.675, 80.325, 80.325);\
         \n  c38: rgb(175.95, 79.05, 79.05);\
         \n  c39: rgb(177.225, 77.775, 77.775);\
         \n  c40: rgb(178.5, 76.5, 76.5);\
         \n  c41: rgb(179.775, 75.225, 75.225);\
         \n  c42: rgb(181.05, 73.95, 73.95);\
         \n  c43: rgb(182.325, 72.675, 72.675);\
         \n  c44: rgb(183.6, 71.4, 71.4);\
         \n  c45: rgb(184.875, 70.125, 70.125);\
         \n  c46: rgb(186.15, 68.85, 68.85);\
         \n  c47: rgb(187.425, 67.575, 67.575);\
         \n  c48: rgb(188.7, 66.3, 66.3);\
         \n  c49: rgb(189.975, 65.025, 65.025);\
         \n  c50: rgb(191.25, 63.75, 63.75);\
         \n  c51: rgb(192.525, 62.475, 62.475);\
         \n  c52: rgb(193.8, 61.2, 61.2);\
         \n  c53: rgb(195.075, 59.925, 59.925);\
         \n  c54: rgb(196.35, 58.65, 58.65);\
         \n  c55: rgb(197.625, 57.375, 57.375);\
         \n  c56: rgb(198.9, 56.1, 56.1);\
         \n  c57: rgb(200.175, 54.825, 54.825);\
         \n  c58: rgb(201.45, 53.55, 53.55);\
         \n  c59: rgb(202.725, 52.275, 52.275);\
         \n  c60: #cc3333;\
         \n  c61: rgb(205.275, 49.725, 49.725);\
         \n  c62: rgb(206.55, 48.45, 48.45);\
         \n  c63: rgb(207.825, 47.175, 47.175);\
         \n  c64: rgb(209.1, 45.9, 45.9);\
         \n  c65: rgb(210.375, 44.625, 44.625);\
         \n  c66: rgb(211.65, 43.35, 43.35);\
         \n  c67: rgb(212.925, 42.075, 42.075);\
         \n  c68: rgb(214.2, 40.8, 40.8);\
         \n  c69: rgb(215.475, 39.525, 39.525);\
         \n  c70: rgb(216.75, 38.25, 38.25);\
         \n  c71: rgb(218.025, 36.975, 36.975);\
         \n  c72: rgb(219.3, 35.7, 35.7);\
         \n  c73: rgb(220.575, 34.425, 34.425);\
         \n  c74: rgb(221.85, 33.15, 33.15);\
         \n  c75: rgb(223.125, 31.875, 31.875);\
         \n  c76: rgb(224.4, 30.6, 30.6);\
         \n  c77: rgb(225.675, 29.325, 29.325);\
         \n  c78: rgb(226.95, 28.05, 28.05);\
         \n  c79: rgb(228.225, 26.775, 26.775);\
         \n  c80: rgb(229.5, 25.5, 25.5);\
         \n  c81: rgb(230.775, 24.225, 24.225);\
         \n  c82: rgb(232.05, 22.95, 22.95);\
         \n  c83: rgb(233.325, 21.675, 21.675);\
         \n  c84: rgb(234.6, 20.4, 20.4);\
         \n  c85: rgb(235.875, 19.125, 19.125);\
         \n  c86: rgb(237.15, 17.85, 17.85);\
         \n  c87: rgb(238.425, 16.575, 16.575);\
         \n  c88: rgb(239.7, 15.3, 15.3);\
         \n  c89: rgb(240.975, 14.025, 14.025);\
         \n  c90: rgb(242.25, 12.75, 12.75);\
         \n  c91: rgb(243.525, 11.475, 11.475);\
         \n  c92: rgb(244.8, 10.2, 10.2);\
         \n  c93: rgb(246.075, 8.925, 8.925);\
         \n  c94: rgb(247.35, 7.65, 7.65);\
         \n  c95: rgb(248.625, 6.375, 6.375);\
         \n  c96: rgb(249.9, 5.1, 5.1);\
         \n  c97: rgb(251.175, 3.825, 3.825);\
         \n  c98: rgb(252.45, 2.55, 2.55);\
         \n  c99: rgb(253.725, 1.275, 1.275);\
         \n  c100: red;\
         \n}\n"
    );
}
