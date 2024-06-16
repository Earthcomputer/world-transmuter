use std::collections::BTreeMap;
use std::sync::OnceLock;
use world_transmuter_engine::DataVersion;

struct VersionData {
    versions_by_id: BTreeMap<u32, Version>,
    versions_by_name: BTreeMap<&'static str, Version>,
    breakpoints: Vec<DataVersion>,
}

impl VersionData {
    fn v(&mut self, name: &'static str, data_version: u32, typ: VersionType) {
        let version = Version {
            name,
            data_version,
            typ,
        };
        self.versions_by_id.insert(data_version, version);
        self.versions_by_name.insert(name, version);
    }
    fn bp(&mut self, version: impl Into<DataVersion>) {
        self.breakpoints.push(version.into());
    }
}

static VERSION_DATA: OnceLock<VersionData> = OnceLock::new();

fn version_data() -> &'static VersionData {
    VERSION_DATA.get_or_init(|| {
        let mut data = VersionData {
            versions_by_id: BTreeMap::new(),
            versions_by_name: BTreeMap::new(),
            breakpoints: Vec::new(),
        };

        data.v("1.8", 99, VersionType::Release);
        data.v("15w32a", 100, VersionType::Snapshot);
        data.v("15w32b", 103, VersionType::Snapshot);
        data.v("15w32c", 104, VersionType::Snapshot);
        data.v("15w33a", 111, VersionType::Snapshot);
        data.v("15w33c", 112, VersionType::Snapshot);
        data.v("15w34a", 114, VersionType::Snapshot);
        data.v("15w34b", 115, VersionType::Snapshot);
        data.v("15w34c", 116, VersionType::Snapshot);
        data.v("15w34d", 117, VersionType::Snapshot);
        data.v("15w35a", 118, VersionType::Snapshot);
        data.v("15w35b", 119, VersionType::Snapshot);
        data.v("15w35c", 120, VersionType::Snapshot);
        data.v("15w35d", 121, VersionType::Snapshot);
        data.v("15w35e", 122, VersionType::Snapshot);
        data.v("15w36a", 123, VersionType::Snapshot);
        data.v("15w36b", 124, VersionType::Snapshot);
        data.v("15w36c", 125, VersionType::Snapshot);
        data.v("15w36d", 126, VersionType::Snapshot);
        data.v("15w37a", 127, VersionType::Snapshot);
        data.v("15w38a", 128, VersionType::Snapshot);
        data.v("15w38b", 129, VersionType::Snapshot);
        data.v("15w39a", 130, VersionType::Snapshot);
        data.v("15w39b", 131, VersionType::Snapshot);
        data.v("15w39c", 132, VersionType::Snapshot);
        data.v("15w40a", 133, VersionType::Snapshot);
        data.v("15w40b", 134, VersionType::Snapshot);
        data.v("15w41a", 136, VersionType::Snapshot);
        data.v("15w41b", 137, VersionType::Snapshot);
        data.v("15w42a", 138, VersionType::Snapshot);
        data.v("15w43a", 139, VersionType::Snapshot);
        data.v("15w43b", 140, VersionType::Snapshot);
        data.v("15w43c", 141, VersionType::Snapshot);
        data.v("15w44a", 142, VersionType::Snapshot);
        data.v("15w44b", 143, VersionType::Snapshot);
        data.v("15w45a", 145, VersionType::Snapshot);
        data.v("15w46a", 146, VersionType::Snapshot);
        data.v("15w47a", 148, VersionType::Snapshot);
        data.v("15w47b", 149, VersionType::Snapshot);
        data.v("15w47c", 150, VersionType::Snapshot);
        data.v("15w49a", 151, VersionType::Snapshot);
        data.v("15w49b", 152, VersionType::Snapshot);
        data.v("15w50a", 153, VersionType::Snapshot);
        data.v("15w51a", 154, VersionType::Snapshot);
        data.v("15w51b", 155, VersionType::Snapshot);
        data.v("16w02a", 156, VersionType::Snapshot);
        data.v("16w03a", 157, VersionType::Snapshot);
        data.v("16w04a", 158, VersionType::Snapshot);
        data.v("16w05a", 159, VersionType::Snapshot);
        data.v("16w05b", 160, VersionType::Snapshot);
        data.v("16w06a", 161, VersionType::Snapshot);
        data.v("16w07a", 162, VersionType::Snapshot);
        data.v("16w07b", 163, VersionType::Snapshot);
        data.v("1.9-pre1", 164, VersionType::Snapshot);
        data.v("1.9-pre2", 165, VersionType::Snapshot);
        data.v("1.9-pre3", 167, VersionType::Snapshot);
        data.v("1.9-pre4", 168, VersionType::Snapshot);
        data.v("1.9", 169, VersionType::Release);
        data.v("1.9.1-pre1", 170, VersionType::Snapshot);
        data.v("1.9.1-pre2", 171, VersionType::Snapshot);
        data.v("1.9.1-pre3", 172, VersionType::Snapshot);
        data.v("1.9.1", 175, VersionType::Release);
        data.v("1.9.2", 176, VersionType::Release);
        data.v("16w14a", 177, VersionType::Snapshot);
        data.v("16w15a", 178, VersionType::Snapshot);
        data.v("16w15b", 179, VersionType::Snapshot);
        data.v("1.9.3-pre1", 180, VersionType::Snapshot);
        data.v("1.9.3-pre2", 181, VersionType::Snapshot);
        data.v("1.9.3-pre3", 182, VersionType::Snapshot);
        data.v("1.9.3", 183, VersionType::Release);
        data.v("1.9.4", 184, VersionType::Release);
        data.v("16w20a", 501, VersionType::Snapshot);
        data.v("16w21a", 503, VersionType::Snapshot);
        data.v("16w21b", 504, VersionType::Snapshot);
        data.v("1.10-pre1", 506, VersionType::Snapshot);
        data.v("1.10-pre2", 507, VersionType::Snapshot);
        data.v("1.10", 510, VersionType::Release);
        data.v("1.10.1", 511, VersionType::Release);
        data.v("1.10.2", 512, VersionType::Release);
        data.v("16w32a", 800, VersionType::Snapshot);
        data.v("16w32b", 801, VersionType::Snapshot);
        data.v("16w33a", 802, VersionType::Snapshot);
        data.v("16w35a", 803, VersionType::Snapshot);
        data.v("16w36a", 805, VersionType::Snapshot);
        data.v("16w38a", 807, VersionType::Snapshot);
        data.v("16w39a", 809, VersionType::Snapshot);
        data.v("16w39b", 811, VersionType::Snapshot);
        data.v("16w39c", 812, VersionType::Snapshot);
        data.v("16w40a", 813, VersionType::Snapshot);
        data.v("16w41a", 814, VersionType::Snapshot);
        data.v("16w42a", 815, VersionType::Snapshot);
        data.v("16w43a", 816, VersionType::Snapshot);
        data.v("16w44a", 817, VersionType::Snapshot);
        data.v("1.11-pre1", 818, VersionType::Snapshot);
        data.v("1.11", 819, VersionType::Release);
        data.v("16w50a", 920, VersionType::Snapshot);
        data.v("1.11.1", 921, VersionType::Release);
        data.v("1.11.2", 922, VersionType::Release);
        data.v("17w06a", 1022, VersionType::Snapshot);
        data.v("17w13a", 1122, VersionType::Snapshot);
        data.v("17w13b", 1123, VersionType::Snapshot);
        data.v("17w14a", 1124, VersionType::Snapshot);
        data.v("17w15a", 1125, VersionType::Snapshot);
        data.v("17w16a", 1126, VersionType::Snapshot);
        data.v("17w16b", 1127, VersionType::Snapshot);
        data.v("17w17a", 1128, VersionType::Snapshot);
        data.v("17w17b", 1129, VersionType::Snapshot);
        data.v("17w18a", 1130, VersionType::Snapshot);
        data.v("17w18b", 1131, VersionType::Snapshot);
        data.v("1.12-pre1", 1132, VersionType::Snapshot);
        data.v("1.12-pre2", 1133, VersionType::Snapshot);
        data.v("1.12-pre3", 1134, VersionType::Snapshot);
        data.v("1.12-pre4", 1135, VersionType::Snapshot);
        data.v("1.12-pre5", 1136, VersionType::Snapshot);
        data.v("1.12-pre6", 1137, VersionType::Snapshot);
        data.v("1.12-pre7", 1138, VersionType::Snapshot);
        data.v("1.12", 1139, VersionType::Release);
        data.v("17w31a", 1239, VersionType::Snapshot);
        data.v("1.12.1-pre1", 1240, VersionType::Snapshot);
        data.v("1.12.1", 1241, VersionType::Release);
        data.v("1.12.2-pre1", 1341, VersionType::Snapshot);
        data.v("1.12.2-pre2", 1342, VersionType::Snapshot);
        data.v("1.12.2", 1343, VersionType::Release);
        data.v("17w43a", 1444, VersionType::Snapshot);
        data.v("17w43b", 1445, VersionType::Snapshot);
        data.v("17w45a", 1447, VersionType::Snapshot);
        data.v("17w45b", 1448, VersionType::Snapshot);
        data.v("17w46a", 1449, VersionType::Snapshot);
        data.bp(1451);
        data.v("17w47a", 1451, VersionType::Snapshot);
        data.bp(1452);
        data.v("17w47b", 1452, VersionType::Snapshot);
        data.v("17w48a", 1453, VersionType::Snapshot);
        data.v("17w49a", 1454, VersionType::Snapshot);
        data.v("17w49b", 1455, VersionType::Snapshot);
        data.v("17w50a", 1457, VersionType::Snapshot);
        data.v("18w01a", 1459, VersionType::Snapshot);
        data.v("18w02a", 1461, VersionType::Snapshot);
        data.v("18w03a", 1462, VersionType::Snapshot);
        data.v("18w03b", 1463, VersionType::Snapshot);
        data.v("18w05a", 1464, VersionType::Snapshot);
        data.v("18w06a", 1466, VersionType::Snapshot);
        data.v("18w07a", 1467, VersionType::Snapshot);
        data.v("18w07b", 1468, VersionType::Snapshot);
        data.v("18w07c", 1469, VersionType::Snapshot);
        data.v("18w08a", 1470, VersionType::Snapshot);
        data.v("18w08b", 1471, VersionType::Snapshot);
        data.v("18w09a", 1472, VersionType::Snapshot);
        data.v("18w10a", 1473, VersionType::Snapshot);
        data.v("18w10b", 1474, VersionType::Snapshot);
        data.v("18w10c", 1476, VersionType::Snapshot);
        data.v("18w10d", 1477, VersionType::Snapshot);
        data.v("18w11a", 1478, VersionType::Snapshot);
        data.v("18w14a", 1479, VersionType::Snapshot);
        data.v("18w14b", 1481, VersionType::Snapshot);
        data.v("18w15a", 1482, VersionType::Snapshot);
        data.v("18w16a", 1483, VersionType::Snapshot);
        data.v("18w19a", 1484, VersionType::Snapshot);
        data.v("18w19b", 1485, VersionType::Snapshot);
        data.v("18w20a", 1489, VersionType::Snapshot);
        data.v("18w20b", 1491, VersionType::Snapshot);
        data.v("18w20c", 1493, VersionType::Snapshot);
        data.v("18w21a", 1495, VersionType::Snapshot);
        data.v("18w21b", 1496, VersionType::Snapshot);
        data.v("18w22a", 1497, VersionType::Snapshot);
        data.v("18w22b", 1498, VersionType::Snapshot);
        data.v("18w22c", 1499, VersionType::Snapshot);
        data.v("1.13-pre1", 1501, VersionType::Snapshot);
        data.v("1.13-pre2", 1502, VersionType::Snapshot);
        data.v("1.13-pre3", 1503, VersionType::Snapshot);
        data.v("1.13-pre4", 1504, VersionType::Snapshot);
        data.v("1.13-pre5", 1511, VersionType::Snapshot);
        data.v("1.13-pre6", 1512, VersionType::Snapshot);
        data.v("1.13-pre7", 1513, VersionType::Snapshot);
        data.v("1.13-pre8", 1516, VersionType::Snapshot);
        data.v("1.13-pre9", 1517, VersionType::Snapshot);
        data.v("1.13-pre10", 1518, VersionType::Snapshot);
        data.v("1.13", 1519, VersionType::Release);
        data.v("18w30a", 1620, VersionType::Snapshot);
        data.v("18w30b", 1621, VersionType::Snapshot);
        data.v("18w31a", 1622, VersionType::Snapshot);
        data.v("18w32a", 1623, VersionType::Snapshot);
        data.v("18w33a", 1625, VersionType::Snapshot);
        data.v("1.13.1-pre1", 1626, VersionType::Snapshot);
        data.v("1.13.1-pre2", 1627, VersionType::Snapshot);
        data.v("1.13.1", 1628, VersionType::Release);
        data.v("1.13.2-pre1", 1629, VersionType::Snapshot);
        data.v("1.13.2-pre2", 1630, VersionType::Snapshot);
        data.v("1.13.2", 1631, VersionType::Release);
        data.v("18w43a", 1901, VersionType::Snapshot);
        data.v("18w43b", 1902, VersionType::Snapshot);
        data.v("18w43c", 1903, VersionType::Snapshot);
        data.v("18w44a", 1907, VersionType::Snapshot);
        data.v("18w45a", 1908, VersionType::Snapshot);
        data.v("18w46a", 1910, VersionType::Snapshot);
        data.v("18w47a", 1912, VersionType::Snapshot);
        data.v("18w47b", 1913, VersionType::Snapshot);
        data.v("18w48a", 1914, VersionType::Snapshot);
        data.v("18w48b", 1915, VersionType::Snapshot);
        data.v("18w49a", 1916, VersionType::Snapshot);
        data.v("18w50a", 1919, VersionType::Snapshot);
        data.v("19w02a", 1921, VersionType::Snapshot);
        data.v("19w03a", 1922, VersionType::Snapshot);
        data.v("19w03b", 1923, VersionType::Snapshot);
        data.v("19w03c", 1924, VersionType::Snapshot);
        data.v("19w04a", 1926, VersionType::Snapshot);
        data.v("19w04b", 1927, VersionType::Snapshot);
        data.v("19w05a", 1930, VersionType::Snapshot);
        data.v("19w06a", 1931, VersionType::Snapshot);
        data.v("19w07a", 1932, VersionType::Snapshot);
        data.v("19w08a", 1933, VersionType::Snapshot);
        data.v("19w08b", 1934, VersionType::Snapshot);
        data.v("19w09a", 1935, VersionType::Snapshot);
        data.v("19w11a", 1937, VersionType::Snapshot);
        data.v("19w11b", 1938, VersionType::Snapshot);
        data.v("19w12a", 1940, VersionType::Snapshot);
        data.v("19w12b", 1941, VersionType::Snapshot);
        data.v("19w13a", 1942, VersionType::Snapshot);
        data.v("3D Shareware v1.34", 1943, VersionType::Snapshot);
        data.v("19w14a", 1944, VersionType::Snapshot);
        data.v("19w14b", 1945, VersionType::Snapshot);
        data.v("1.14-pre1", 1947, VersionType::Snapshot);
        data.v("1.14-pre2", 1948, VersionType::Snapshot);
        data.v("1.14-pre3", 1949, VersionType::Snapshot);
        data.v("1.14-pre4", 1950, VersionType::Snapshot);
        data.v("1.14-pre5", 1951, VersionType::Snapshot);
        data.v("1.14", 1952, VersionType::Release);
        data.v("1.14.1-pre1", 1955, VersionType::Snapshot);
        data.v("1.14.1-pre2", 1956, VersionType::Snapshot);
        data.v("1.14.1", 1957, VersionType::Release);
        data.v("1.14.2-pre1", 1958, VersionType::Snapshot);
        data.v("1.14.2-pre2", 1959, VersionType::Snapshot);
        data.v("1.14.2-pre3", 1960, VersionType::Snapshot);
        data.v("1.14.2-pre4", 1962, VersionType::Snapshot);
        data.v("1.14.2", 1963, VersionType::Release);
        data.v("1.14.3-pre1", 1964, VersionType::Snapshot);
        data.v("1.14.3-pre2", 1965, VersionType::Snapshot);
        data.v("1.14.3-pre3", 1966, VersionType::Snapshot);
        data.v("1.14.3-pre4", 1967, VersionType::Snapshot);
        data.v("1.14.3", 1968, VersionType::Release);
        data.v("1.14.4-pre1", 1969, VersionType::Snapshot);
        data.v("1.14.4-pre2", 1970, VersionType::Snapshot);
        data.v("1.14.4-pre3", 1971, VersionType::Snapshot);
        data.v("1.14.4-pre4", 1972, VersionType::Snapshot);
        data.v("1.14.4-pre5", 1973, VersionType::Snapshot);
        data.v("1.14.4-pre6", 1974, VersionType::Snapshot);
        data.v("1.14.4-pre7", 1975, VersionType::Snapshot);
        data.v("1.14.4", 1976, VersionType::Release);
        data.v("19w34a", 2200, VersionType::Snapshot);
        data.v("19w35a", 2201, VersionType::Snapshot);
        data.v("19w36a", 2203, VersionType::Snapshot);
        data.v("19w37a", 2204, VersionType::Snapshot);
        data.v("19w38a", 2205, VersionType::Snapshot);
        data.v("19w38b", 2206, VersionType::Snapshot);
        data.v("19w39a", 2207, VersionType::Snapshot);
        data.v("19w40a", 2208, VersionType::Snapshot);
        data.v("19w41a", 2210, VersionType::Snapshot);
        data.v("19w42a", 2212, VersionType::Snapshot);
        data.v("19w44a", 2213, VersionType::Snapshot);
        data.v("19w45a", 2214, VersionType::Snapshot);
        data.v("19w45b", 2215, VersionType::Snapshot);
        data.v("19w46a", 2216, VersionType::Snapshot);
        data.v("19w46b", 2217, VersionType::Snapshot);
        data.v("1.15-pre1", 2218, VersionType::Snapshot);
        data.v("1.15-pre2", 2219, VersionType::Snapshot);
        data.v("1.15-pre3", 2220, VersionType::Snapshot);
        data.v("1.15-pre4", 2221, VersionType::Snapshot);
        data.v("1.15-pre5", 2222, VersionType::Snapshot);
        data.v("1.15-pre6", 2223, VersionType::Snapshot);
        data.v("1.15-pre7", 2224, VersionType::Snapshot);
        data.v("1.15", 2225, VersionType::Release);
        data.v("1.15.1-pre1", 2226, VersionType::Snapshot);
        data.v("1.15.1", 2227, VersionType::Release);
        data.v("1.15.2-pre1", 2228, VersionType::Snapshot);
        data.v("1.15.2-pre2", 2229, VersionType::Snapshot);
        data.v("1.15.2", 2230, VersionType::Release);
        data.v("20w06a", 2504, VersionType::Snapshot);
        data.v("20w07a", 2506, VersionType::Snapshot);
        data.v("20w08a", 2507, VersionType::Snapshot);
        data.v("20w09a", 2510, VersionType::Snapshot);
        data.v("20w10a", 2512, VersionType::Snapshot);
        data.v("20w11a", 2513, VersionType::Snapshot);
        data.v("20w12a", 2515, VersionType::Snapshot);
        data.v("20w13a", 2520, VersionType::Snapshot);
        data.v("20w13b", 2521, VersionType::Snapshot);
        data.v("20w14infinite", 2522, VersionType::Snapshot);
        data.v("20w14a", 2524, VersionType::Snapshot);
        data.v("20w15a", 2525, VersionType::Snapshot);
        data.v("20w16a", 2526, VersionType::Snapshot);
        data.v("20w17a", 2529, VersionType::Snapshot);
        data.v("20w18a", 2532, VersionType::Snapshot);
        data.v("20w19a", 2534, VersionType::Snapshot);
        data.v("20w20a", 2536, VersionType::Snapshot);
        data.v("20w20b", 2537, VersionType::Snapshot);
        data.v("20w21a", 2554, VersionType::Snapshot);
        data.v("20w22a", 2555, VersionType::Snapshot);
        data.v("1.16-pre1", 2556, VersionType::Snapshot);
        data.v("1.16-pre2", 2557, VersionType::Snapshot);
        data.v("1.16-pre3", 2559, VersionType::Snapshot);
        data.v("1.16-pre4", 2560, VersionType::Snapshot);
        data.v("1.16-pre5", 2561, VersionType::Snapshot);
        data.v("1.16-pre6", 2562, VersionType::Snapshot);
        data.v("1.16-pre7", 2563, VersionType::Snapshot);
        data.v("1.16-pre8", 2564, VersionType::Snapshot);
        data.v("1.16-rc1", 2565, VersionType::Snapshot);
        data.v("1.16", 2566, VersionType::Release);
        data.v("1.16.1", 2567, VersionType::Release);
        data.v("20w27a", 2569, VersionType::Snapshot);
        data.v("20w28a", 2570, VersionType::Snapshot);
        data.v("20w29a", 2571, VersionType::Snapshot);
        data.v("20w30a", 2572, VersionType::Snapshot);
        data.v("1.16.2-pre1", 2573, VersionType::Snapshot);
        data.v("1.16.2-pre2", 2574, VersionType::Snapshot);
        data.v("1.16.2-pre3", 2575, VersionType::Snapshot);
        data.v("1.16.2-rc1", 2576, VersionType::Snapshot);
        data.v("1.16.2-rc2", 2577, VersionType::Snapshot);
        data.v("1.16.2", 2578, VersionType::Release);
        data.v("1.16.3-rc1", 2579, VersionType::Snapshot);
        data.v("1.16.3", 2580, VersionType::Release);
        data.v("1.16.4-pre1", 2581, VersionType::Snapshot);
        data.v("1.16.4-pre2", 2582, VersionType::Snapshot);
        data.v("1.16.4-rc1", 2583, VersionType::Snapshot);
        data.v("1.16.4", 2584, VersionType::Release);
        data.v("1.16.5-rc1", 2585, VersionType::Snapshot);
        data.v("1.16.5", 2586, VersionType::Release);
        data.v("20w45a", 2681, VersionType::Snapshot);
        data.v("20w46a", 2682, VersionType::Snapshot);
        data.v("20w48a", 2683, VersionType::Snapshot);
        data.v("20w49a", 2685, VersionType::Snapshot);
        data.v("20w51a", 2687, VersionType::Snapshot);
        data.v("21w03a", 2689, VersionType::Snapshot);
        data.v("21w05a", 2690, VersionType::Snapshot);
        data.v("21w05b", 2692, VersionType::Snapshot);
        data.v("21w06a", 2694, VersionType::Snapshot);
        data.v("21w07a", 2695, VersionType::Snapshot);
        data.v("21w08a", 2697, VersionType::Snapshot);
        data.v("21w08b", 2698, VersionType::Snapshot);
        data.v("21w10a", 2699, VersionType::Snapshot);
        data.v("21w11a", 2703, VersionType::Snapshot);
        data.v("21w13a", 2705, VersionType::Snapshot);
        data.v("21w14a", 2706, VersionType::Snapshot);
        data.v("21w15a", 2709, VersionType::Snapshot);
        data.v("21w16a", 2711, VersionType::Snapshot);
        data.v("21w17a", 2712, VersionType::Snapshot);
        data.v("21w18a", 2713, VersionType::Snapshot);
        data.v("21w19a", 2714, VersionType::Snapshot);
        data.v("21w20a", 2715, VersionType::Snapshot);
        data.v("1.17-pre1", 2716, VersionType::Snapshot);
        data.v("1.17-pre2", 2718, VersionType::Snapshot);
        data.v("1.17-pre3", 2719, VersionType::Snapshot);
        data.v("1.17-pre4", 2720, VersionType::Snapshot);
        data.v("1.17-pre5", 2721, VersionType::Snapshot);
        data.v("1.17-rc1", 2722, VersionType::Snapshot);
        data.v("1.17-rc2", 2723, VersionType::Snapshot);
        data.v("1.17", 2724, VersionType::Release);
        data.v("1.17.1-pre1", 2725, VersionType::Snapshot);
        data.v("1.17.1-pre2", 2726, VersionType::Snapshot);
        data.v("1.17.1-pre3", 2727, VersionType::Snapshot);
        data.v("1.17.1-rc1", 2728, VersionType::Snapshot);
        data.v("1.17.1-rc2", 2729, VersionType::Snapshot);
        data.v("1.17.1", 2730, VersionType::Release);
        data.bp(2731);
        data.v("21w37a", 2834, VersionType::Snapshot);
        data.v("21w38a", 2835, VersionType::Snapshot);
        data.v("21w39a", 2836, VersionType::Snapshot);
        data.v("21w40a", 2838, VersionType::Snapshot);
        data.v("21w41a", 2839, VersionType::Snapshot);
        data.v("21w42a", 2840, VersionType::Snapshot);
        data.v("21w43a", 2844, VersionType::Snapshot);
        data.v("21w44a", 2845, VersionType::Snapshot);
        data.v("1.18-pre1", 2847, VersionType::Snapshot);
        data.v("1.18-pre2", 2848, VersionType::Snapshot);
        data.v("1.18-pre3", 2849, VersionType::Snapshot);
        data.v("1.18-pre4", 2850, VersionType::Snapshot);
        data.v("1.18-pre5", 2851, VersionType::Snapshot);
        data.v("1.18-pre6", 2853, VersionType::Snapshot);
        data.v("1.18-pre7", 2854, VersionType::Snapshot);
        data.v("1.18-pre8", 2855, VersionType::Snapshot);
        data.v("1.18-rc1", 2856, VersionType::Snapshot);
        data.v("1.18-rc2", 2857, VersionType::Snapshot);
        data.v("1.18-rc3", 2858, VersionType::Snapshot);
        data.v("1.18-rc4", 2859, VersionType::Snapshot);
        data.v("1.18", 2860, VersionType::Release);
        data.v("1.18.1-pre1", 2861, VersionType::Snapshot);
        data.v("1.18.1-rc1", 2862, VersionType::Snapshot);
        data.v("1.18.1-rc2", 2863, VersionType::Snapshot);
        data.v("1.18.1-rc3", 2864, VersionType::Snapshot);
        data.v("1.18.1", 2865, VersionType::Release);
        data.v("22w03a", 2966, VersionType::Snapshot);
        data.v("22w05a", 2967, VersionType::Snapshot);
        data.v("22w06a", 2968, VersionType::Snapshot);
        data.v("22w07a", 2969, VersionType::Snapshot);
        data.v("1.18.2-pre1", 2971, VersionType::Snapshot);
        data.v("1.18.2-pre2", 2972, VersionType::Snapshot);
        data.v("1.18.2-pre3", 2973, VersionType::Snapshot);
        data.v("1.18.2-rc1", 2974, VersionType::Snapshot);
        data.v("1.18.2", 2975, VersionType::Release);
        data.bp(2976);
        data.v("22w13oneblockatatime", 3076, VersionType::Snapshot);
        data.v("22w11a", 3080, VersionType::Snapshot);
        data.v("22w12a", 3082, VersionType::Snapshot);
        data.v("22w13a", 3085, VersionType::Snapshot);
        data.v("22w14a", 3088, VersionType::Snapshot);
        data.v("22w15a", 3089, VersionType::Snapshot);
        data.v("22w16a", 3091, VersionType::Snapshot);
        data.v("22w16b", 3092, VersionType::Snapshot);
        data.v("22w17a", 3093, VersionType::Snapshot);
        data.v("22w18a", 3095, VersionType::Snapshot);
        data.v("22w19a", 3096, VersionType::Snapshot);
        data.v("1.19-pre1", 3098, VersionType::Snapshot);
        data.v("1.19-pre2", 3099, VersionType::Snapshot);
        data.v("1.19-pre3", 3100, VersionType::Snapshot);
        data.v("1.19-pre4", 3101, VersionType::Snapshot);
        data.v("1.19-pre5", 3102, VersionType::Snapshot);
        data.v("1.19-rc1", 3103, VersionType::Snapshot);
        data.v("1.19-rc2", 3104, VersionType::Snapshot);
        data.v("1.19", 3105, VersionType::Release);
        data.v("22w24a", 3106, VersionType::Snapshot);
        data.v("1.19.1-pre1", 3107, VersionType::Snapshot);
        data.v("1.19.1-rc1", 3109, VersionType::Snapshot);
        data.v("1.19.1-pre2", 3110, VersionType::Snapshot);
        data.v("1.19.1-pre3", 3111, VersionType::Snapshot);
        data.v("1.19.1-pre4", 3112, VersionType::Snapshot);
        data.v("1.19.1-pre5", 3113, VersionType::Snapshot);
        data.v("1.19.1-pre6", 3114, VersionType::Snapshot);
        data.v("1.19.1-rc2", 3115, VersionType::Snapshot);
        data.v("1.19.1-rc3", 3116, VersionType::Snapshot);
        data.v("1.19.1", 3117, VersionType::Release);
        data.v("1.19.2-rc1", 3118, VersionType::Snapshot);
        data.v("1.19.2-rc2", 3119, VersionType::Snapshot);
        data.v("1.19.2", 3120, VersionType::Release);
        data.v("22w42a", 3205, VersionType::Snapshot);
        data.v("22w43a", 3206, VersionType::Snapshot);
        data.v("22w44a", 3207, VersionType::Snapshot);
        data.v("22w45a", 3208, VersionType::Snapshot);
        data.v("22w46a", 3210, VersionType::Snapshot);
        data.v("1.19.3-pre1", 3211, VersionType::Snapshot);
        data.v("1.19.3-pre2", 3212, VersionType::Snapshot);
        data.v("1.19.3-pre3", 3213, VersionType::Snapshot);
        data.v("1.19.3-rc1", 3215, VersionType::Snapshot);
        data.v("1.19.3-rc2", 3216, VersionType::Snapshot);
        data.v("1.19.3-rc3", 3217, VersionType::Snapshot);
        data.v("1.19.3", 3218, VersionType::Release);
        data.v("23w03a", 3320, VersionType::Snapshot);
        data.v("23w04a", 3321, VersionType::Snapshot);
        data.v("23w05a", 3323, VersionType::Snapshot);
        data.v("23w06a", 3326, VersionType::Snapshot);
        data.v("23w07a", 3329, VersionType::Snapshot);
        data.v("1.19.4-pre1", 3330, VersionType::Snapshot);
        data.v("1.19.4-pre2", 3331, VersionType::Snapshot);
        data.v("1.19.4-pre3", 3332, VersionType::Snapshot);
        data.v("1.19.4-pre4", 3333, VersionType::Snapshot);
        data.v("1.19.4-rc1", 3334, VersionType::Snapshot);
        data.v("1.19.4-rc2", 3335, VersionType::Snapshot);
        data.v("1.19.4-rc3", 3336, VersionType::Snapshot);
        data.v("1.19.4", 3337, VersionType::Release);
        data.bp(3338);
        data.v("23w12a", 3442, VersionType::Snapshot);
        data.v("23w13a", 3443, VersionType::Snapshot);
        data.v("23w13a_or_b", 3444, VersionType::Snapshot);
        data.v("23w14a", 3445, VersionType::Snapshot);
        data.v("23w16a", 3449, VersionType::Snapshot);
        data.v("23w17a", 3452, VersionType::Snapshot);
        data.v("23w18a", 3453, VersionType::Snapshot);
        data.v("1.20-pre1", 3454, VersionType::Snapshot);
        data.v("1.20-pre2", 3455, VersionType::Snapshot);
        data.v("1.20-pre3", 3456, VersionType::Snapshot);
        data.v("1.20-pre4", 3457, VersionType::Snapshot);
        data.v("1.20-pre5", 3458, VersionType::Snapshot);
        data.v("1.20-pre6", 3460, VersionType::Snapshot);
        data.v("1.20-pre7", 3461, VersionType::Snapshot);
        data.v("1.20-rc1", 3462, VersionType::Snapshot);
        data.v("1.20", 3463, VersionType::Release);
        data.v("1.20.1-rc1", 3464, VersionType::Snapshot);
        data.v("1.20.1", 3465, VersionType::Release);
        data.v("23w31a", 3567, VersionType::Snapshot);
        data.v("23w32a", 3569, VersionType::Snapshot);
        data.v("23w33a", 3570, VersionType::Snapshot);
        data.v("23w35a", 3571, VersionType::Snapshot);
        data.v("1.20.2-pre1", 3572, VersionType::Snapshot);
        data.v("1.20.2-pre2", 3573, VersionType::Snapshot);
        data.v("1.20.2-pre3", 3574, VersionType::Snapshot);
        data.v("1.20.2-pre4", 3575, VersionType::Snapshot);
        data.v("1.20.2-rc1", 3576, VersionType::Snapshot);
        data.v("1.20.2-rc2", 3577, VersionType::Snapshot);
        data.v("1.20.2", 3578, VersionType::Release);
        data.v("23w40a", 3679, VersionType::Snapshot);
        data.v("23w41a", 3681, VersionType::Snapshot);
        data.v("23w42a", 3684, VersionType::Snapshot);
        data.v("23w43a", 3686, VersionType::Snapshot);
        data.v("23w43b", 3687, VersionType::Snapshot);
        data.v("23w44a", 3688, VersionType::Snapshot);
        data.v("23w45a", 3690, VersionType::Snapshot);
        data.v("23w46a", 3691, VersionType::Snapshot);
        data.v("1.20.3-pre1", 3693, VersionType::Snapshot);
        data.v("1.20.3-pre2", 3694, VersionType::Snapshot);
        data.v("1.20.3-pre3", 3695, VersionType::Snapshot);
        data.v("1.20.3-pre4", 3696, VersionType::Snapshot);
        data.v("1.20.3-rc1", 3697, VersionType::Snapshot);
        data.v("1.20.3", 3698, VersionType::Release);
        data.v("1.20.4-rc1", 3699, VersionType::Snapshot);
        data.v("1.20.4", 3700, VersionType::Release);
        data.v("23w51a", 3801, VersionType::Snapshot);
        data.v("23w51b", 3802, VersionType::Snapshot);
        data.v("24w03a", 3804, VersionType::Snapshot);
        data.v("24w03b", 3805, VersionType::Snapshot);
        data.v("24w04a", 3806, VersionType::Snapshot);
        data.v("24w05a", 3809, VersionType::Snapshot);
        data.v("24w05b", 3811, VersionType::Snapshot);
        data.v("24w06a", 3815, VersionType::Snapshot);
        data.v("24w07a", 3817, VersionType::Snapshot);
        data.bp(DataVersion::new(3818, 5));
        data.bp(3819);
        data.v("24w09a", 3819, VersionType::Snapshot);
        data.v("24w10a", 3821, VersionType::Snapshot);
        data.v("24w11a", 3823, VersionType::Snapshot);
        data.v("24w14potato", 3824, VersionType::Snapshot);
        data.v("24w13a", 3826, VersionType::Snapshot);
        data.v("24w14a", 3827, VersionType::Snapshot);
        data.v("1.20.5-pre1", 3829, VersionType::Snapshot);
        data.v("1.20.5-pre2", 3830, VersionType::Snapshot);
        data.v("1.20.5-pre3", 3831, VersionType::Snapshot);
        data.v("1.20.5-pre4", 3832, VersionType::Snapshot);
        data.v("1.20.5-rc1", 3834, VersionType::Snapshot);
        data.v("1.20.5-rc2", 3835, VersionType::Snapshot);
        data.v("1.20.5-rc3", 3836, VersionType::Snapshot);
        data.v("1.20.5", 3837, VersionType::Release);
        data.v("1.20.6-rc1", 3838, VersionType::Snapshot);
        data.v("1.20.6", 3839, VersionType::Release);
        data.bp(3840);

        data
    })
}

pub fn get_version_by_id(id: u32) -> Option<Version> {
    version_data().versions_by_id.get(&id).copied()
}

pub fn get_version_by_name(name: &str) -> Option<Version> {
    version_data().versions_by_name.get(name).copied()
}

pub fn get_versions() -> impl DoubleEndedIterator<Item = Version> {
    version_data().versions_by_id.values().copied()
}

pub fn get_breakpoints() -> &'static [DataVersion] {
    &version_data().breakpoints
}

#[derive(Debug, Copy, Clone, Eq)]
#[non_exhaustive]
pub struct Version {
    pub name: &'static str,
    pub data_version: u32,
    pub typ: VersionType,
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.data_version == other.data_version
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VersionType {
    Release,
    Snapshot,
}

#[cfg(test)]
#[cfg(feature = "update_checks")]
mod tests {
    use crate::version_names::{get_version_by_id, VersionType};
    use serde::Deserialize;
    use time::macros::datetime;
    use time::OffsetDateTime;
    use zip::ZipArchive;

    #[test]
    fn scrape_versions() {
        #[derive(Deserialize)]
        struct Version {
            id: String,
            #[serde(rename = "type")]
            typ: String,
            url: String,
            #[serde(rename = "releaseTime", with = "time::serde::iso8601")]
            release_time: OffsetDateTime,
        }

        #[derive(Deserialize)]
        struct VersionManifest {
            versions: Vec<Version>,
        }

        #[derive(Deserialize)]
        struct DownloadsEntry {
            url: String,
        }

        #[derive(Deserialize)]
        struct Downloads {
            client: DownloadsEntry,
        }

        #[derive(Deserialize)]
        struct ClientJson {
            downloads: Downloads,
        }

        #[derive(Deserialize)]
        struct GameVersionJson {
            world_version: u32,
        }

        let mut manifest: VersionManifest =
            attohttpc::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
                .send()
                .expect("failed to download version manifest")
                .json()
                .expect("json error in manifest");

        let time_1_20_6 = datetime!(2024-04-29 0:00 UTC);
        manifest
            .versions
            .retain(|version| version.release_time >= time_1_20_6);

        let mut missing_versions = Vec::new();

        for version in manifest.versions {
            println!("Checking {}", version.id);

            let client_json: ClientJson = attohttpc::get(version.url)
                .send()
                .expect("failed to download client json")
                .json()
                .expect("json error in client json");

            let client_jar = attohttpc::get(client_json.downloads.client.url)
                .send()
                .expect("failed to download client jar")
                .bytes()
                .expect("failed to download client jar");
            let mut client_zip = ZipArchive::new(std::io::Cursor::new(client_jar))
                .expect("client jar is invalid zip");
            let mut data_version = None;
            for i in 0..client_zip.len() {
                let zip_entry = client_zip.by_index(i).expect("invalid entry in client jar");
                if zip_entry.name() == "version.json" {
                    let game_version_json: GameVersionJson =
                        serde_json::from_reader(zip_entry).expect("invalid game version json");
                    data_version = Some(game_version_json.world_version);
                    break;
                }
            }

            let data_version = data_version.expect("missing data version");
            if get_version_by_id(data_version).is_none() {
                missing_versions.push(crate::version_names::Version {
                    name: version.id.leak(),
                    data_version,
                    typ: if version.typ == "release" {
                        VersionType::Release
                    } else {
                        VersionType::Snapshot
                    },
                });
            }
        }

        missing_versions.sort_by_key(|version| version.data_version);
        let mut current_version = 0;
        missing_versions.retain(|version| {
            let result = version.data_version != current_version;
            current_version = version.data_version;
            result
        });

        assert!(
            missing_versions.is_empty(),
            "There are {} missing versions! Consider adding the following code (and don't forget breakpoints!):\n{}",
            missing_versions.len(),
            missing_versions
                .iter()
                .map(|version| format!(
                    "data.v(\"{}\", {}, VersionType::{});",
                    version.name,
                    version.data_version,
                    if version.typ == VersionType::Release {
                        "Release"
                    } else {
                        "Snapshot"
                    }
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
