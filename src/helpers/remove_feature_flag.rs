use crate::helpers::mc_namespace_map::McNamespaceSet;
use world_transmuter_engine::{DataVersion, JCompound, JList, JValue, MapDataConverterFunc};

pub(crate) struct ConverterRemoveFeatureFlags<'a>(McNamespaceSet<'a>);

impl<'a> ConverterRemoveFeatureFlags<'a> {
    pub(crate) fn new(flags: McNamespaceSet<'a>) -> Self {
        ConverterRemoveFeatureFlags(flags)
    }
}

impl MapDataConverterFunc for ConverterRemoveFeatureFlags<'_> {
    fn convert(&self, data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
        let Some(JValue::List(enabled_features_list)) = data.get_mut("enabled_features") else {
            return;
        };
        let JList::String(enabled_features) = enabled_features_list else {
            return;
        };
        let mut removed_features = Vec::new();

        enabled_features.retain_mut(|feature| {
            if self.0.contains(&feature[..]) {
                removed_features.push(std::mem::take(feature));
                false
            } else {
                true
            }
        });
        if enabled_features.is_empty() {
            *enabled_features_list = JList::End;
        }

        if !removed_features.is_empty() {
            data.insert("removed_features", JList::String(removed_features));
        }
    }
}
