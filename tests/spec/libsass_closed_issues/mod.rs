//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues"

mod t47_str_slice;

mod issue_2640;

mod issue_2681;

mod issue_100;

mod issue_1007;

mod issue_1016;

mod issue_1021;

mod issue_1025;

mod issue_1030;

mod issue_1036;

mod issue_1043;

mod issue_1060;

mod issue_1061;

mod issue_1063;

mod issue_1074;

mod issue_1075;

mod issue_1079;

mod issue_108;

mod issue_1080;

mod issue_1081;

mod issue_1082;

mod issue_1086;

mod issue_1087;

mod issue_1091;

mod issue_1092;

mod issue_1093;

mod issue_1098;

mod issue_1101;

mod issue_1102;

mod issue_1103;

mod issue_1106;

mod issue_1107;

mod issue_1115;

mod issue_112;

mod issue_1121;

mod issue_1127;

mod issue_113;

mod issue_1130;

mod issue_1132;

mod issue_1133;

mod issue_1153;

mod issue_1162;

mod issue_1163;

mod issue_1167;

mod issue_1168;

mod issue_1169;

mod issue_1170;

mod issue_1171;

mod issue_1178;

mod issue_1187;

mod issue_1188;

// From "sass-spec/spec/libsass-closed-issues/issue_1192"
#[test]
#[ignore] // wrong result
fn issue_1192() {
    assert_eq!(
        crate::rsass(
            "$keyword: foobar;\
            \n\
            \n@mixin test($arglist...){\
            \n  $map: keywords($arglist);\
            \n  /*#{inspect($map)}*/\
            \n  /*#{inspect($arglist)}*/\
            \n}\
            \n\
            \n// Works\
            \n@include test(foo, bar, baz);\
            \n// Ruby Sass:  /*foo, bar, baz*/\
            \n// LibSass  :  /*foo, bar, baz*/\
            \n\
            \n// LibSass does not inspect as ()\
            \n@include test;\
            \n// Ruby Sass:  /*()*/\
            \n// LibSass  :  /**/\
            \n\
            \n// Ruby Sass throws error – LibSass shows keywords in arglist\
            \n// (keywords should not show in arglist – see below)\
            \n@include test(foo, bar, baz, $keyword: keyword);\
            \n// Ruby Sass:  \"Mixin test1 doesn\'t have an argument named $keyword.\"\
            \n// LibSass  :  /*foo, bar, baz, $keyword: keyword*/"
        )
        .unwrap(),
        "/*()*/\
        \n/*foo, bar, baz*/\
        \n/*()*/\
        \n/*()*/\
        \n/*(keyword: keyword)*/\
        \n/*foo, bar, baz*/\
        \n"
    );
}

mod issue_1206;

mod issue_1207;

mod issue_1208;

mod issue_1210;

mod issue_1214;

mod issue_1215;

mod issue_1216;

mod issue_1218;

mod issue_1224;

mod issue_1230;

// From "sass-spec/spec/libsass-closed-issues/issue_1231"
#[test]
fn issue_1231() {
    assert_eq!(
        crate::rsass(
            "div::before {\
            \n  content: #{\"\\\"\"+\\e600+\"\\\"\"};\
            \n}"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \ndiv::before {\
        \n  content: \"\u{e600}\";\
        \n}\
        \n"
    );
}

mod issue_1233;

mod issue_1240;

mod issue_1243;

mod issue_1248;

mod issue_1251;

mod issue_1253;

mod issue_1255;

mod issue_1257;

mod issue_1258;

mod issue_1259;

mod issue_1260;

mod issue_1263;

mod issue_1266;

mod issue_1269;

mod issue_1271;

mod issue_1273;

mod issue_1277;

mod issue_1279;

mod issue_1281;

mod issue_1283;

mod issue_1285;

mod issue_1291;

mod issue_1294;

mod issue_1295;

mod issue_1297;

mod issue_1298;

mod issue_1301;

mod issue_1303;

mod issue_1304;

mod issue_1305;

mod issue_131;

mod issue_1322;

mod issue_1323;

mod issue_1328;

mod issue_1331;

mod issue_1332;

mod issue_1333;

mod issue_1336;

mod issue_1355;

mod issue_137;

mod issue_1370;

mod issue_1376;

mod issue_1393;

mod issue_1394;

mod issue_1396;

mod issue_1398;

mod issue_1399;

mod issue_1404;

mod issue_1405;

mod issue_1413;

mod issue_1415;

mod issue_1417;

mod issue_1418;

mod issue_1419;

mod issue_1422;

mod issue_143;

mod issue_1432;

mod issue_1434;

mod issue_1437;

mod issue_1438;

mod issue_1440;

mod issue_1441;

mod issue_1448;

mod issue_1452;

mod issue_1459;

mod issue_1482;

mod issue_1484;

mod issue_1486;

mod issue_1487;

mod issue_1488;

mod issue_151;

mod issue_152;

mod issue_1526;

mod issue_1527;

mod issue_1535;

mod issue_1537;

mod issue_154;

mod issue_1546;

mod issue_1550;

mod issue_1557;

mod issue_1566;

mod issue_1567;

mod issue_1568;

mod issue_1569;

mod issue_1570;

mod issue_1574;

mod issue_1577;

mod issue_1578;

mod issue_1579;

mod issue_1583;

mod issue_1584;

mod issue_1585;

mod issue_1590;

mod issue_1596;

mod issue_1601;

mod issue_1604;

mod issue_1610;

mod issue_1612;

mod issue_1622;

mod issue_1624;

mod issue_1629;

mod issue_1632;

mod issue_1634;

mod issue_1640;

mod issue_1644;

mod issue_1645;

mod issue_1647;

mod issue_1648;

mod issue_1650;

mod issue_1651;

mod issue_1654;

mod issue_1658;

mod issue_1667;

mod issue_1669;

mod issue_167;

mod issue_1670;

mod issue_1671;

mod issue_1672;

mod issue_1673;

mod issue_1681;

mod issue_1683;

mod issue_1685;

mod issue_1706;

mod issue_1709;

mod issue_1710;

mod issue_1715;

mod issue_1720;

mod issue_1722;

mod issue_1723;

mod issue_1726;

mod issue_1729;

mod issue_1732;

mod issue_1733;

mod issue_1739;

mod issue_1741;

mod issue_1752;

mod issue_1757;

mod issue_1765;

mod issue_1766;

mod issue_1768;

mod issue_1770;

mod issue_1776;

// From "sass-spec/spec/libsass-closed-issues/issue_1786"
#[test]
#[ignore] // wrong result
fn issue_1786() {
    assert_eq!(
        crate::rsass(
            "$input: \"\\0_\\a_\\A\";\
            \n\
            \ntest {\
            \n    bug1: \"#{\"_\\a\" + b}\";\
            \n    bug2: \"#{a $input}\";\
            \n}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \ntest {\
        \n  bug1: \"_\\a b\";\
        \n  bug2: \"a �_ _ \";\
        \n}\
        \n"
    );
}

mod issue_1792;

mod issue_1793;

mod issue_1794;

mod issue_1796;

mod issue_1797;

mod issue_1798;

mod issue_1801;

mod issue_1803;

mod issue_1804;

mod issue_1812;

mod issue_1813;

mod issue_1819;

mod issue_1822;

mod issue_1825;

mod issue_1839;

mod issue_185;

mod issue_1873;

mod issue_1886;

mod issue_1889;

mod issue_1890;

mod issue_1898;

mod issue_1901;

mod issue_1904;

mod issue_1907;

mod issue_1915;

mod issue_1916;

mod issue_192;

mod issue_1923;

mod issue_1926;

mod issue_1927;

mod issue_1931;

mod issue_1940;

mod issue_1941;

mod issue_1944;

mod issue_1945;

mod issue_1947;

mod issue_1960;

mod issue_1969;

mod issue_1971;

// From "sass-spec/spec/libsass-closed-issues/issue_1977"
#[test]
fn issue_1977() {
    assert_eq!(
        crate::rsass(
            "body#some-\\(selector\\) {\
            \ncolor: red;\
            \n}\
            \n\
            \n#äöü  {\
            \n  color: reds;\
            \n}"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \nbody#some-\\(selector\\) {\
        \n  color: red;\
        \n}\
        \n#äöü {\
        \n  color: reds;\
        \n}\
        \n"
    );
}

mod issue_1991;

mod issue_1993;

mod issue_1994;

mod issue_1996;

mod issue_2000;

mod issue_2006;

mod issue_2007;

mod issue_2009;

mod issue_201;

mod issue_2017;

mod issue_2020;

mod issue_2023;

mod issue_2031;

mod issue_2034;

mod issue_2042;

mod issue_2053;

mod issue_2054;

mod issue_2055;

mod issue_2056;

mod issue_2057;

mod issue_2074;

mod issue_2081;

mod issue_2095;

mod issue_2106;

mod issue_2112;

mod issue_2116;

// From "sass-spec/spec/libsass-closed-issues/issue_2120"
#[test]
fn issue_2120() {
    assert_eq!(
        crate::rsass("@import url(//xyz.cöm/ürl)").unwrap(),
        "@charset \"UTF-8\";\
        \n@import url(//xyz.cöm/ürl);\
        \n"
    );
}

mod issue_2123;

mod issue_2124;

mod issue_2139;

mod issue_2140;

mod issue_2143;

mod issue_2147;

mod issue_2149;

mod issue_2150;

mod issue_2153;

mod issue_2154;

mod issue_2155;

mod issue_2156;

mod issue_2169;

mod issue_2175;

mod issue_2177;

mod issue_2179;

mod issue_2185;

mod issue_2198;

mod issue_2200;

mod issue_2202;

mod issue_2205;

mod issue_221255;

mod issue_221289;

mod issue_2233;

mod issue_224;

mod issue_2243;

mod issue_2246;

mod issue_2260;

mod issue_2261;

mod issue_2289;

mod issue_2291;

mod issue_2295;

mod issue_2303;

mod issue_2304;

mod issue_2307;

mod issue_2309;

mod issue_231;

// From "sass-spec/spec/libsass-closed-issues/issue_2320"
#[test]
fn issue_2320() {
    assert_eq!(
        crate::rsass(
            "$char-f: \'\\66\';\r\
            \n$char-g: \'\\67\';\r\
            \n\r\
            \n.test-1 {\r\
            \n  content: \'#{$char-f}\\feff\';\r\
            \n}\r\
            \n\r\
            \n.test-2 {\r\
            \n  content: \'#{$char-g}\\feff\';\r\
            \n}\r\
            \n\r\
            \n// this is broken\r\
            \n.test-3 {\r\
            \n  content: \'\\feff#{$char-f}\';\r\
            \n}\r\
            \n\r\
            \n.test-4 {\r\
            \n  content: \'\\feff#{$char-g}\';\r\
            \n}"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \n.test-1 {\
        \n  content: \"f\u{feff}\";\
        \n}\
        \n.test-2 {\
        \n  content: \"g\u{feff}\";\
        \n}\
        \n.test-3 {\
        \n  content: \"\u{feff}f\";\
        \n}\
        \n.test-4 {\
        \n  content: \"\u{feff}g\";\
        \n}\
        \n"
    );
}

mod issue_2321;

mod issue_2330;

mod issue_2333;

mod issue_2341;

mod issue_2346;

mod issue_2347;

mod issue_2349;

mod issue_2352;

mod issue_2358;

mod issue_2360;

mod issue_2365;

mod issue_2366;

mod issue_2369;

mod issue_2371;

mod issue_2374;

mod issue_2376;

mod issue_2382;

mod issue_238760;

mod issue_239;

mod issue_2394;

mod issue_2399;

mod issue_2429;

mod issue_2444;

// Ignoring "issue_2446", tests with expected error not implemented yet.

// Ignoring "issue_245443", tests with expected error not implemented yet.

mod issue_246;

mod issue_2462;

mod issue_2464;

mod issue_2465;

mod issue_2467;

mod issue_2468;

mod issue_2472;

mod issue_2480;

mod issue_2482;

mod issue_2509;

mod issue_2520;

mod issue_254;

mod issue_2560;

mod issue_2569;

mod issue_257;

mod issue_2582;

mod issue_2625;

mod issue_2633;

mod issue_267;

mod issue_2697;

mod issue_274;

mod issue_2779;

mod issue_279;

mod issue_2808;

mod issue_2863;

mod issue_2884;

mod issue_289;

mod issue_2959;

mod issue_2975;

mod issue_2980;

mod issue_2994;

mod issue_308;

mod issue_309;

mod issue_312;

mod issue_338;

mod issue_344;

mod issue_346;

mod issue_349;

mod issue_368;

mod issue_394;

mod issue_424;

mod issue_435;

mod issue_439;

mod issue_442;

mod issue_45;

mod issue_453;

mod issue_456;

mod issue_469;

mod issue_472;

mod issue_478;

mod issue_485;

mod issue_487;

mod issue_492;

mod issue_495;

mod issue_502;

mod issue_506;

mod issue_509;

mod issue_510;

mod issue_512;

mod issue_534;

mod issue_535;

mod issue_54;

mod issue_548;

mod issue_549;

mod issue_550;

mod issue_552;

mod issue_553;

mod issue_555;

mod issue_556;

mod issue_557;

mod issue_558;

mod issue_56;

mod issue_574;

mod issue_575;

mod issue_577;

mod issue_578;

mod issue_579;

mod issue_58;

mod issue_59;

mod issue_590;

mod issue_592;

mod issue_593;

mod issue_595;

mod issue_6;

// From "sass-spec/spec/libsass-closed-issues/issue_602"
#[test]
fn issue_602() {
    assert_eq!(
        crate::rsass(
            "#foo.\\bar {\
            \n  color: red;\
            \n}\
            \n\
            \n#foo.b\\ar {\
            \n  color: red;\
            \n}\
            \n\
            \n#foo\\.bar {\
            \n  color: red;\
            \n}\
            \n\
            \n#foo\\bar {\
            \n  color: red;\
            \n}\
            \n\
            \n#fo\\o.bar {\
            \n  color: red;\
            \n}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \n#foo.ºr {\
        \n  color: red;\
        \n}\
        \n#foo.b\\a r {\
        \n  color: red;\
        \n}\
        \n#foo\\.bar {\
        \n  color: red;\
        \n}\
        \n#fooºr {\
        \n  color: red;\
        \n}\
        \n#foo.bar {\
        \n  color: red;\
        \n}\
        \n"
    );
}

mod issue_610;

mod issue_613;

mod issue_615;

mod issue_622;

mod issue_623;

mod issue_628;

mod issue_63;

mod issue_639;

mod issue_64;

mod issue_641;

mod issue_643;

mod issue_644;

mod issue_646;

mod issue_652;

mod issue_659;

mod issue_660;

mod issue_666;

mod issue_67;

mod issue_672;

mod issue_673;

mod issue_674;

mod issue_683;

mod issue_688;

mod issue_690;

mod issue_694;

mod issue_698;

mod issue_699;

mod issue_700;

mod issue_701;

mod issue_702;

mod issue_703;

mod issue_708;

mod issue_712;

mod issue_713;

mod issue_72;

mod issue_73;

mod issue_733;

mod issue_736;

mod issue_738;

mod issue_740;

mod issue_748;

mod issue_759;

mod issue_760;

mod issue_763;

mod issue_77;

// From "sass-spec/spec/libsass-closed-issues/issue_783"
#[test]
fn issue_783() {
    assert_eq!(
        crate::rsass(
            "// $a: 12px / 1em;\
            \n// $b: 6px / 1em;\
            \n// $c: 10em;\
            \n// $x: -9999em;\
            \n// $aa: 1px * 1px;\
            \n\
            \na {\
            \n  $foo: 2em;\
            \n  $bar: 2em;\
            \n\
            \n  foo: $foo;          // 2em  ✔\
            \n  bar: $bar;          // 2em  ✔\
            \n  // a: $foo * $bar;     // 4em*em isn\'t a valid CSS value.  ✔\
            \n  a: $foo / $bar;     // 1  ✔\
            \n  a: $foo + $bar;     // 4em  ✔\
            \n  a: $foo - $bar;     // 0em  ✔\
            \n\
            \n\
            \n  $foo: 2px;\
            \n  $bar: 2em;\
            \n\
            \n  foo: $foo;          // 2px  ✔\
            \n  bar: $bar;          // 2em  ✔\
            \n  // a: $foo * $bar;     // 4em*px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo / $bar;     // 1px/em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'px\'.  ✔\
            \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'px\'.  ✔\
            \n\
            \n\
            \n  $foo: 2em;\
            \n  $bar: 2px;\
            \n\
            \n  foo: $foo;          // 2em  ✔\
            \n  bar: $bar;          // 2px  ✔\
            \n  // a: $foo * $bar;     // 4em*px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo / $bar;     // 1em/px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'px\' and \'em\'.  ✔\
            \n  // a: $foo - $bar;     // Incompatible units: \'px\' and \'em\'.  ✔\
            \n\
            \n\
            \n  $foo: 2px / 2em;\
            \n  $bar: 2px;\
            \n\
            \n  // foo: $foo;          // 1px/em isn\'t a valid CSS value.  ✔\
            \n  bar: $bar;          // 2px  ✔\
            \n  // a: $foo * $bar;     // 2px*px/em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo / $bar;     // 0.5/em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'\' and \'em\'.\
            \n  // a: $foo - $bar;     // Incompatible units: \'\' and \'em\'.\
            \n\
            \n\
            \n  $foo: 2em / 2px;\
            \n  $bar: 2px;\
            \n\
            \n  // foo: $foo;          // 1em/px isn\'t a valid CSS value.  ✔\
            \n  bar: $bar;          // 2px  ✔\
            \n  a: $foo * $bar;     // 2em  ✔\
            \n  // a: $foo / $bar;     // 0.5em/px*px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'px\' and \'em\'.\
            \n  // a: $foo - $bar;     // Incompatible units: \'px\' and \'em\'.\
            \n\
            \n\
            \n  $foo: 2em / 2px;\
            \n  $bar: 2em / 2px;\
            \n\
            \n  // foo: $foo;          // 1em/px isn\'t a valid CSS value.  ✔\
            \n  // bar: $bar;          // 1em/px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo * $bar;     // 1em*em/px*px isn\'t a valid CSS value.  ✔\
            \n  a: $foo / $bar;     // 1  ✔\
            \n  // a: $foo + $bar;     // 2em/px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo - $bar;     // 0em/px isn\'t a valid CSS value.  ✔\
            \n\
            \n\
            \n  $foo: 2px / 2em;\
            \n  $bar: 2em / 2px;\
            \n\
            \n  // foo: $foo;          // 1px/em isn\'t a valid CSS value.  ✔\
            \n  // bar: $bar;          // 1em/px isn\'t a valid CSS value.  ✔\
            \n  a: $foo * $bar;     // 1  ✔\
            \n  // a: $foo / $bar;     // 1px*px/em*em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'px\'.\
            \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'px\'.\
            \n\
            \n\
            \n  $foo: 2px;\
            \n  $bar: 2px / 2em;\
            \n\
            \n  foo: $foo;          // 2px  ✔\
            \n  // bar: $bar;          // 1px/em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo * $bar;     // 2px*px/em isn\'t a valid CSS value.  ✔\
            \n  a: $foo / $bar;     // 2em  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'\'.\
            \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'\'.\
            \n\
            \n\
            \n  $foo: 2px;\
            \n  $bar: 2em / 2px;\
            \n\
            \n  foo: $foo;          // 2px  ✔\
            \n  // bar: $bar;          // 1em/px isn\'t a valid CSS value.  ✔\
            \n  a: $foo * $bar;     // 2em  ✔\
            \n  // a: $foo / $bar;     // 2px*px/em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'px\'.\
            \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'px\'.\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: 2em;\
        \n  bar: 2em;\
        \n  a: 1;\
        \n  a: 4em;\
        \n  a: 0em;\
        \n  foo: 2px;\
        \n  bar: 2em;\
        \n  foo: 2em;\
        \n  bar: 2px;\
        \n  bar: 2px;\
        \n  bar: 2px;\
        \n  a: 2em;\
        \n  a: 1;\
        \n  a: 1;\
        \n  foo: 2px;\
        \n  a: 2em;\
        \n  foo: 2px;\
        \n  a: 2em;\
        \n}\
        \n"
    );
}

mod issue_784;

mod issue_803;

mod issue_813;

mod issue_815;

mod issue_817;

// From "sass-spec/spec/libsass-closed-issues/issue_820"
#[test]
fn issue_820() {
    assert_eq!(
        crate::rsass(
            "@charset \"UTF-8\";\
            \n/*!  Force output of above line by adding a unicode character. ♫ */\
            \nhtml, body {\
            \n  height: 100%; }\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \n/*!  Force output of above line by adding a unicode character. ♫ */\
        \nhtml, body {\
        \n  height: 100%;\
        \n}\
        \n"
    );
}

mod issue_823;

mod issue_828;

mod issue_829;

mod issue_83;

mod issue_845;

mod issue_857;

mod issue_859;

mod issue_86;

mod issue_864;

mod issue_87;

mod issue_870;

mod issue_871;

mod issue_873;

mod issue_877;

mod issue_883;

mod issue_884;

mod issue_890;

mod issue_893;

mod issue_894;

mod issue_91;

mod issue_930;

mod issue_931;

mod issue_941;

mod issue_942;

mod issue_943;

mod issue_945;

mod issue_947;

mod issue_948;

mod issue_950;

mod issue_976;

mod issue_978;

mod issue_980;

mod issue_988;

mod issue_992;
