use crate::helpers::resource_location::ResourceLocation;
use crate::{static_string_mc_set, types};
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{DataVersion, JCompound, JValue, MapDataConverterFunc};

const VERSION: u32 = 3088;

static_string_mc_set! {
    STATUSES_TO_SKIP_BLENDING, statuses_to_skip_blending, {
        "empty",
        "structure_starts",
        "structure_references",
        "biomes",
    }
}

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(VERSION, ConverterAddBlendingData);
}

pub(crate) struct ConverterAddBlendingData;

fn create_blending_data(height: i32, min_y: i32) -> JCompound {
    jcompound! {
        "min_section" => min_y >> 4,
        "max_section" => (min_y + height) >> 4,
    }
}

impl MapDataConverterFunc for ConverterAddBlendingData {
    fn convert(&self, data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
        data.remove("blending_data");
        let Some(JValue::Compound(context)) = data.get("__context") else {
            return;
        };
        let Some(JValue::String(dimension)) = context.get("dimension") else {
            return;
        };
        if dimension != "minecraft:overworld" {
            return;
        }
        let Some(JValue::String(status)) = data.get("Status") else {
            return;
        };
        let status = ResourceLocation::make_correct(status);

        if !statuses_to_skip_blending().contains(&status[..]) {
            data.insert("blending_data", create_blending_data(384, -64));
        } else if let Some(JValue::Compound(below_zero_retrogen)) = data.get("below_zero_retrogen")
        {
            if let Some(JValue::String(real_status)) = below_zero_retrogen.get("target_status") {
                let real_status = ResourceLocation::make_correct(real_status);
                if !statuses_to_skip_blending().contains(&real_status[..]) {
                    data.insert("blending_data", create_blending_data(256, 0));
                }
            }
        }
    }
}
