use rust_dataconverter_engine::{IdDataType, MapDataType, ObjectDataType};
use std::cell::{Ref, RefCell};
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub(crate) struct MinecraftTypesMut<'a>(&'a MinecraftTypes<'a>);

pub struct MinecraftTypes<'a>(MinecraftTypesInner, PhantomData<RefCell<&'a ()>>);

macro_rules! define_minecraft_types {
    ($($field_name:ident : $type:ident ($name:literal)),* $(,)?) => {

        impl<'a> MinecraftTypesMut<'a> {
            $(
                pub(crate) fn $field_name(self) -> &'a RefCell<$type<'a>> {
                    // SAFETY: the "actual" type of the field is RefCell<$type<'a>>, see the comment
                    // inside MinecraftTypesInner.
                    unsafe {
                        std::mem::transmute::<&'a RefCell<$type<'static>>, &'a RefCell<$type<'a>>>(&self.0.0.$field_name)
                    }
                }
            )*
        }

        struct MinecraftTypesInner {
            // SAFETY: although these types are declared as RefCell<$type<'static>>, their actual
            // type is RefCell<$type<'a>> where 'a is the lifetime of the struct. This is enforced
            // by the fields only being accessed via MinecraftTypesMut::$field_name(), which
            // transmutes directly to RefCell<$type<'a>>.
            // So, why do this? It's to bypass the drop check, which essentially checks that the
            // destructor does not follow any references to the self-referential struct which may
            // be partially deleted. This is only possible via the dyn traits inside the $type's,
            // and there is no way to enforce this, so this struct in isolation is technically
            // unsound. However, this struct can only be accessed mutably from within this crate,
            // meaning no-one from outside could add a naughty self-referential drop implementation.
            // All the implementations from inside this crate promise not to add self-referential
            // drop impls, meaning this struct in the context of this crate as a whole is sound.
            // This is not ideal, but it's the only way I could get this self-referencing struct to
            // work, and I'm so fucking done with refactoring it.
            $(
                $field_name: RefCell<$type<'static>>,
            )*
        }

        impl<'a> MinecraftTypes<'a> {
            $(
                pub fn $field_name(&'a self) -> Ref<'a, $type<'a>> {
                    MinecraftTypesMut(self).$field_name().borrow()
                }
            )*

            pub fn create_empty() -> Self {
                Self(
                    MinecraftTypesInner {
                        $(
                            $field_name: RefCell::new($type::new($name)),
                        )*
                    },
                    PhantomData,
                )
            }
        }
    }
}

define_minecraft_types! {
    level: MapDataType("Level"),
    player: MapDataType("Player"),
    chunk: MapDataType("Chunk"),
    hotbar: MapDataType("Hotbar"),
    options: MapDataType("Options"),
    structure: MapDataType("Structure"),
    stats: MapDataType("Stats"),
    saved_data: MapDataType("SavedData"),
    advancements: MapDataType("Advancements"),
    poi_chunk: MapDataType("PoiChunk"),
    entity_chunk: MapDataType("EntityChunk"),
    tile_entity: IdDataType("TileEntity"),
    item_stack: IdDataType("ItemStack"),
    block_state: MapDataType("BlockName"),
    entity_name: ObjectDataType("EntityName"),
    entity: IdDataType("Entity"),
    block_name: ObjectDataType("BlockName"),
    item_name: ObjectDataType("ItemName"),
    untagged_spawner: MapDataType("Spawner"),
    structure_feature: MapDataType("StructureFeature"),
    objective: MapDataType("Objective"),
    team: MapDataType("Team"),
    recipe: ObjectDataType("RecipeName"),
    biome: ObjectDataType("Biome"),
    world_gen_settings: MapDataType("WorldGenSettings"),
    game_event_name: ObjectDataType("GameEventName"),

    multi_noise_biome_source_parameter_list: ObjectDataType("MULTI_NOISE_BIOME_SOURCE_PARAMETER_LIST"),
}

impl<'a> MinecraftTypes<'a> {
    pub fn register_versions(&'a self) {
        // General notes:
        // - Structure converters run before everything.
        // - ID specific converters run after structure converters.
        // - Structure walkers run after id specific converters.
        // - ID specific walkers run after structure walkers.
        crate::versions::register_versions(MinecraftTypesMut(self));
    }
}
