macro_rules! versions {
    ($(mod $version:ident);* $(;)?) => {
        $(
            pub(crate) mod $version;
        )*

        pub(crate) fn register_versions<T: rust_dataconverter_engine::Types + ?Sized>(types: &$crate::types::MinecraftTypesMut<T>) {
            $(
                $version::register(types);
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
}
