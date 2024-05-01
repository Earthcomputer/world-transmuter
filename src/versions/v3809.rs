use crate::types;
use world_transmuter_engine::{DataVersion, JCompound, JList, JValue, MapDataConverterFunc};

const VERSION: u32 = 3809;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id("minecraft:llama", VERSION, SlotConverter);
    types::entity_mut().add_converter_for_id("minecraft:mule", VERSION, SlotConverter);
    types::entity_mut().add_converter_for_id("minecraft:donkey", VERSION, SlotConverter);
}

struct SlotConverter;

impl MapDataConverterFunc for SlotConverter {
    fn convert(&self, data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
        let Some(JValue::List(JList::Compound(items))) = data.get_mut("Items") else {
            return;
        };
        for item in items {
            if let Some(slot) = item.get_mut("Slot") {
                let slot_value = slot.as_i8().unwrap_or(0);
                *slot = JValue::Byte(slot_value - 2);
            }
        }
    }
}
