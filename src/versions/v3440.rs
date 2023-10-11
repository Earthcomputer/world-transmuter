use crate::helpers::mc_namespace_map::McNamespaceSet;
use crate::helpers::remove_feature_flag::ConverterRemoveFeatureFlags;
use crate::helpers::resource_location::ResourceLocation;
use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{value_data_converter_func, JValueMut};

const VERSION: u32 = 3440;

pub(crate) fn register() {
    // Note: MULTI_NOISE_BIOME_SOURCE_PARAMETER_LIST is namespaced string
    types::multi_noise_biome_source_parameter_list_mut().add_structure_converter(
        VERSION,
        value_data_converter_func(|data, _from_version, _to_version| {
            if let JValueMut::String(data) = data {
                let corrected_name = ResourceLocation::parse(data)
                    .map_or_else(|_| (*data).clone(), |rl| rl.to_java_string());
                if corrected_name == "minecraft:overworld_update_1_20" {
                    **data = JavaString::from("minecraft:overworld");
                }
            }
        }),
    );
    types::level_mut().add_structure_converter(
        VERSION,
        ConverterRemoveFeatureFlags::new({
            let mut features = McNamespaceSet::new();
            features.insert_mc("update_1_20");
            features
        }),
    );
}
