use crate::helpers::mc_namespace_map::McNamespaceSet;
use crate::helpers::remove_feature_flag::ConverterRemoveFeatureFlags;
use crate::types;

const VERSION: u32 = 3939;

pub(crate) fn register() {
    types::level_mut().add_structure_converter(
        VERSION,
        ConverterRemoveFeatureFlags::new({
            let mut flags = McNamespaceSet::new();
            flags.insert_mc("update_1_21");
            flags
        }),
    )
}
