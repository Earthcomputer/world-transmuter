#[macro_export]
macro_rules! static_string_set {
    ($name:ident = {$($values:literal),* $(,)?}) => {
        fn $name() -> &'static ::std::collections::BTreeSet<&'static ::java_string::JavaStr> {
            static VALUE: ::std::sync::OnceLock<::std::collections::BTreeSet<&::java_string::JavaStr>> = ::std::sync::OnceLock::new();
            VALUE.get_or_init(|| {
                let mut set = ::std::collections::BTreeSet::new();
                $(
                set.insert(::java_string::JavaStr::from_str($values));
                )*
                set
            })
        }
    }
}

#[macro_export]
macro_rules! static_string_map {
    ($name:ident = {$($keys:literal => $values:literal),* $(,)?}) => {
        fn $name() -> &'static ::std::collections::BTreeMap<&'static ::java_string::JavaStr, &'static ::java_string::JavaStr> {
            static VALUE: ::std::sync::OnceLock<::std::collections::BTreeMap<&::java_string::JavaStr, &::java_string::JavaStr>> = ::std::sync::OnceLock::new();
            VALUE.get_or_init(|| {
                let mut map = ::std::collections::BTreeMap::new();
                $(
                map.insert(::java_string::JavaStr::from_str($keys), ::java_string::JavaStr::from_str($values));
                )*
                map
            })
        }
    }
}

#[macro_export]
macro_rules! static_string_mc_set {
    ($name:ident = {$($values:literal),* $(,)?}) => {
        fn $name() -> &'static $crate::helpers::mc_namespace_map::McNamespaceSet<'static> {
            static VALUE: ::std::sync::OnceLock<$crate::helpers::mc_namespace_map::McNamespaceSet> = ::std::sync::OnceLock::new();
            VALUE.get_or_init(|| {
                let mut set = $crate::helpers::mc_namespace_map::McNamespaceSet::new();
                $(
                assert!(!$values.starts_with("minecraft:"));
                set.insert_mc($values);
                )*
                set
            })
        }
    }
}

#[macro_export]
macro_rules! static_string_mc_map {
    ($name:ident = {$($keys:literal => $values:literal),* $(,)?}) => {
        fn $name() -> &'static $crate::helpers::mc_namespace_map::McNamespaceMap<'static, &'static ::java_string::JavaStr> {
            static VALUE: ::std::sync::OnceLock<$crate::helpers::mc_namespace_map::McNamespaceMap<&::java_string::JavaStr>> = ::std::sync::OnceLock::new();
            VALUE.get_or_init(|| {
                let mut map = $crate::helpers::mc_namespace_map::McNamespaceMap::new();
                $(
                assert!(!$keys.starts_with("minecraft:"));
                map.insert_mc($keys, ::java_string::JavaStr::from_str($values));
                )*
                map
            })
        }
    }
}
