use crate::types;
use crate::versions::v3807;
use java_string::JavaStr;
use world_transmuter_engine::{
    map_data_converter_func, rename_key, DataVersion, JCompound, JList, JValue,
    MapDataConverterFunc,
};

const VERSION: u32 = 3813;

const PATROLLING_MOBS: [&JavaStr; 6] = [
    JavaStr::from_str("minecraft:witch"),
    JavaStr::from_str("minecraft:ravager"),
    JavaStr::from_str("minecraft:pillager"),
    JavaStr::from_str("minecraft:illusioner"),
    JavaStr::from_str("minecraft:evoker"),
    JavaStr::from_str("minecraft:vindicator"),
];

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:bee",
        VERSION,
        RootPositionConverter::new([("HivePos", "hive_pos"), ("FlowerPos", "flower_pos")]),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:end_crystal",
        VERSION,
        RootPositionConverter::new([("BeamTarget", "beam_target")]),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:wandering_trader",
        VERSION,
        RootPositionConverter::new([("WanderTarget", "wander_target")]),
    );
    for id in PATROLLING_MOBS {
        types::entity_mut().add_converter_for_id(
            id,
            VERSION,
            RootPositionConverter::new([("PatrolTarget", "patrol_target")]),
        );
    }

    types::entity_mut()
        .add_structure_converter(VERSION, RootPositionConverter::new([("Leash", "leash")]));

    types::tile_entity_mut().add_converter_for_id(
        "minecraft:beehive",
        VERSION,
        RootPositionConverter::new([("FlowerPos", "flower_pos")]),
    );
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:end_gateway",
        VERSION,
        RootPositionConverter::new([("ExitPortal", "exit_portal")]),
    );

    types::saved_data_map_data_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(data)) = data.get_mut("data") else {
                return;
            };

            if let Some(JValue::List(JList::Compound(frames))) = data.get_mut("frames") {
                for frame in frames {
                    v3807::flatten_block_pos(frame, "Pos");

                    rename_key(frame, "Pos", "pos");
                    rename_key(frame, "Rotation", "rotation");
                    rename_key(frame, "EntityId", "entity_id");
                }
            }

            if let Some(JValue::List(JList::Compound(banners))) = data.get_mut("banners") {
                for banner in banners {
                    rename_key(banner, "Pos", "pos");
                    rename_key(banner, "Color", "color");
                    rename_key(banner, "Name", "name");
                }
            }
        }),
    );

    types::item_stack_mut().add_converter_for_id(
        "minecraft:compass",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(tag)) = data.get_mut("tag") else {
                return;
            };
            v3807::flatten_block_pos(tag, "LodestonePos");
        }),
    );
}

struct RootPositionConverter<const N: usize> {
    convert: [(&'static JavaStr, &'static JavaStr); N],
}

impl<const N: usize> RootPositionConverter<N> {
    fn new<T: AsRef<JavaStr> + ?Sized>(convert: [(&'static T, &'static T); N]) -> Self {
        Self {
            convert: convert.map(|(from, to)| (from.as_ref(), to.as_ref())),
        }
    }
}

impl<const N: usize> MapDataConverterFunc for RootPositionConverter<N> {
    fn convert(&self, data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
        for (from, to) in self.convert {
            v3807::flatten_block_pos(data, from);
            rename_key(data, from, to);
        }
    }
}
