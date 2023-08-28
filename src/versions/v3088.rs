use crate::helpers::mc_namespace_map::McNamespaceSet;
use crate::helpers::resource_location::ResourceLocation;
use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::{DataVersion, MapDataConverterFunc};
use std::sync::OnceLock;
use valence_nbt::{compound, Compound, Value};

const VERSION: u32 = 3088;

static STATUSES_TO_SKIP_BLENDING: OnceLock<McNamespaceSet> = OnceLock::new();

fn statuses_to_skip_blending() -> &'static McNamespaceSet<'static> {
    STATUSES_TO_SKIP_BLENDING.get_or_init(|| {
        let mut set = McNamespaceSet::new();
        set.insert_mc("empty");
        set.insert_mc("structure_starts");
        set.insert_mc("structure_references");
        set.insert_mc("biomes");
        set
    })
}

pub(crate) fn register(types: &MinecraftTypesMut) {
    types
        .chunk
        .borrow_mut()
        .add_structure_converter(VERSION, ConverterAddBlendingData);
}

pub(crate) struct ConverterAddBlendingData;

fn create_blending_data(height: i32, min_y: i32) -> Compound {
    compound! {
        "min_section" => min_y >> 4,
        "max_section" => (min_y + height) >> 4,
    }
}

impl MapDataConverterFunc for ConverterAddBlendingData {
    fn convert(&self, data: &mut Compound, _from_version: DataVersion, _to_version: DataVersion) {
        data.remove("blending_data");
        let Some(Value::Compound(context)) = data.get("__context") else {
            return;
        };
        let Some(Value::String(dimension)) = context.get("dimension") else {
            return;
        };
        if dimension != "minecraft:overworld" {
            return;
        }
        let Some(Value::String(status)) = data.get("Status") else {
            return;
        };
        let status = status
            .parse::<ResourceLocation>()
            .map_or_else(|_| status.clone(), |rl| rl.to_string());

        if !statuses_to_skip_blending().contains(&status[..]) {
            data.insert("blending_data", create_blending_data(384, -64));
        } else if let Some(Value::Compound(below_zero_retrogen)) = data.get("below_zero_retrogen") {
            if let Some(Value::String(real_status)) = below_zero_retrogen.get("target_status") {
                let real_status = real_status
                    .parse::<ResourceLocation>()
                    .map_or_else(|_| real_status.clone(), |rl| rl.to_string());
                if !statuses_to_skip_blending().contains(&real_status[..]) {
                    data.insert("blending_data", create_blending_data(256, 0));
                }
            }
        }
    }
}
