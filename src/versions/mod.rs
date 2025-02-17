macro_rules! versions {
    ($(mod $version:ident);* $(;)?) => {
        $(
            pub(crate) mod $version;
        )*

        pub(crate) fn register_versions() {
            $(
                $version::register();
            )*
        }
    }
}

versions! {
    mod v99;
    mod v100;
    mod v101;
    mod v102;
    mod v105;
    mod v106;
    mod v107;
    mod v108;
    mod v109;
    mod v110;
    mod v111;
    mod v113;
    mod v135;
    mod v143;
    mod v147;
    mod v165;
    mod v501;
    mod v502;
    mod v505;
    mod v700;
    mod v701;
    mod v702;
    mod v703;
    mod v704;
    mod v705;
    mod v804;
    mod v806;
    mod v808;
    mod v813;
    mod v816;
    mod v820;
    mod v1022;
    mod v1125;
    mod v1344;
    mod v1446;
    mod v1450;
    mod v1451;
    mod v1456;
    mod v1458;
    mod v1460;
    mod v1466;
    mod v1470;
    mod v1474;
    mod v1475;
    mod v1480;
    mod v1483;
    mod v1484;
    mod v1486;
    mod v1487;
    mod v1488;
    mod v1490;
    mod v1492;
    mod v1494;
    mod v1496;
    mod v1500;
    mod v1501;
    mod v1502;
    mod v1506;
    mod v1510;
    mod v1514;
    mod v1515;
    mod v1624;
    mod v1800;
    mod v1801;
    mod v1802;
    mod v1803;
    mod v1904;
    mod v1905;
    mod v1906;
    mod v1909;
    mod v1911;
    mod v1914;
    mod v1917;
    mod v1918;
    mod v1920;
    mod v1925;
    mod v1928;
    mod v1929;
    mod v1931;
    mod v1936;
    mod v1946;
    mod v1948;
    mod v1953;
    mod v1955;
    mod v1961;
    mod v1963;
    mod v2100;
    mod v2202;
    mod v2209;
    mod v2211;
    mod v2218;
    mod v2501;
    mod v2502;
    mod v2503;
    mod v2505;
    mod v2508;
    mod v2509;
    mod v2511;
    mod v2514;
    mod v2516;
    mod v2518;
    mod v2519;
    mod v2522;
    mod v2523;
    mod v2527;
    mod v2528;
    mod v2529;
    mod v2531;
    mod v2533;
    mod v2535;
    mod v2538;
    mod v2550;
    mod v2551;
    mod v2552;
    mod v2553;
    mod v2558;
    mod v2568;
    mod v2671;
    mod v2679;
    mod v2680;
    mod v2684;
    mod v2686;
    mod v2688;
    mod v2690;
    mod v2691;
    mod v2693;
    mod v2696;
    mod v2700;
    mod v2701;
    mod v2702;
    mod v2707;
    mod v2710;
    mod v2717;
    mod v2825;
    mod v2831;
    mod v2832;
    mod v2833;
    mod v2838;
    mod v2841;
    mod v2842;
    mod v2843;
    mod v2846;
    mod v2852;
    mod v2967;
    mod v2970;
    mod v3077;
    mod v3078;
    mod v3081;
    mod v3082;
    mod v3083;
    mod v3084;
    mod v3086;
    mod v3087;
    mod v3088;
    mod v3090;
    mod v3093;
    mod v3094;
    mod v3097;
    mod v3108;
    mod v3201;
    mod v3203;
    mod v3204;
    mod v3209;
    mod v3214;
    mod v3319;
    mod v3322;
    mod v3325;
    mod v3326;
    mod v3327;
    mod v3328;
    mod v3438;
    mod v3439;
    mod v3440;
    mod v3441;
    mod v3447;
    mod v3448;
    mod v3450;
    mod v3451;
    mod v3459;
    mod v3564;
    mod v3565;
    mod v3566;
    mod v3568;
    mod v3682;
    mod v3683;
    mod v3685;
    mod v3689;
    mod v3692;
    mod v3799;
    mod v3800;
    mod v3803;
    mod v3807;
    mod v3808;
    mod v3809;
    mod v3812;
    mod v3813;
    mod v3814;
    mod v3816;
    mod v3818;
    mod v3820;
    mod v3825;
    mod v3828;
    mod v3833;
    mod v3938;
    mod v3939;
    mod v3943;
    mod v3945;
    mod v4054;
    mod v4055;
    mod v4057;
    mod v4059;
}
