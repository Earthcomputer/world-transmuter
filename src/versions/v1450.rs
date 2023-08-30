use crate::helpers::block_flattening_v1450;
use crate::MinecraftTypes;
use rust_dataconverter_engine::map_data_converter_func;

const VERSION: u32 = 1450;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.block_state.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(new_data) = block_flattening_v1450::flatten_nbt(data) {
                *data = new_data;
            }
        }),
    );
}
