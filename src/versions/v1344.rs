use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1344;

static BUTTON_ID_TO_NAME: SyncOnceCell<rust_dataconverter_engine::Map<u8, String>> = SyncOnceCell::new();

fn button_id_to_name() -> &'static rust_dataconverter_engine::Map<u8, String> {
    BUTTON_ID_TO_NAME.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert(0, "key.unknown".to_owned());
        map.insert(11, "key.0".to_owned());
        map.insert(2, "key.1".to_owned());
        map.insert(3, "key.2".to_owned());
        map.insert(4, "key.3".to_owned());
        map.insert(5, "key.4".to_owned());
        map.insert(6, "key.5".to_owned());
        map.insert(7, "key.6".to_owned());
        map.insert(8, "key.7".to_owned());
        map.insert(9, "key.8".to_owned());
        map.insert(10, "key.9".to_owned());
        map.insert(30, "key.a".to_owned());
        map.insert(40, "key.apostrophe".to_owned());
        map.insert(48, "key.b".to_owned());
        map.insert(43, "key.backslash".to_owned());
        map.insert(14, "key.backspace".to_owned());
        map.insert(46, "key.c".to_owned());
        map.insert(58, "key.caps.lock".to_owned());
        map.insert(51, "key.comma".to_owned());
        map.insert(32, "key.d".to_owned());
        map.insert(211, "key.delete".to_owned());
        map.insert(208, "key.down".to_owned());
        map.insert(18, "key.e".to_owned());
        map.insert(207, "key.end".to_owned());
        map.insert(28, "key.enter".to_owned());
        map.insert(13, "key.equal".to_owned());
        map.insert(1, "key.escape".to_owned());
        map.insert(33, "key.f".to_owned());
        map.insert(59, "key.f1".to_owned());
        map.insert(68, "key.f10".to_owned());
        map.insert(87, "key.f11".to_owned());
        map.insert(88, "key.f12".to_owned());
        map.insert(100, "key.f13".to_owned());
        map.insert(101, "key.f14".to_owned());
        map.insert(102, "key.f15".to_owned());
        map.insert(103, "key.f16".to_owned());
        map.insert(104, "key.f17".to_owned());
        map.insert(105, "key.f18".to_owned());
        map.insert(113, "key.f19".to_owned());
        map.insert(60, "key.f2".to_owned());
        map.insert(61, "key.f3".to_owned());
        map.insert(62, "key.f4".to_owned());
        map.insert(63, "key.f5".to_owned());
        map.insert(64, "key.f6".to_owned());
        map.insert(65, "key.f7".to_owned());
        map.insert(66, "key.f8".to_owned());
        map.insert(67, "key.f9".to_owned());
        map.insert(34, "key.g".to_owned());
        map.insert(41, "key.grave.accent".to_owned());
        map.insert(35, "key.h".to_owned());
        map.insert(199, "key.home".to_owned());
        map.insert(23, "key.i".to_owned());
        map.insert(210, "key.insert".to_owned());
        map.insert(36, "key.j".to_owned());
        map.insert(37, "key.k".to_owned());
        map.insert(82, "key.keypad.0".to_owned());
        map.insert(79, "key.keypad.1".to_owned());
        map.insert(80, "key.keypad.2".to_owned());
        map.insert(81, "key.keypad.3".to_owned());
        map.insert(75, "key.keypad.4".to_owned());
        map.insert(76, "key.keypad.5".to_owned());
        map.insert(77, "key.keypad.6".to_owned());
        map.insert(71, "key.keypad.7".to_owned());
        map.insert(72, "key.keypad.8".to_owned());
        map.insert(73, "key.keypad.9".to_owned());
        map.insert(78, "key.keypad.add".to_owned());
        map.insert(83, "key.keypad.decimal".to_owned());
        map.insert(181, "key.keypad.divide".to_owned());
        map.insert(156, "key.keypad.enter".to_owned());
        map.insert(141, "key.keypad.equal".to_owned());
        map.insert(55, "key.keypad.multiply".to_owned());
        map.insert(74, "key.keypad.subtract".to_owned());
        map.insert(38, "key.l".to_owned());
        map.insert(203, "key.left".to_owned());
        map.insert(56, "key.left.alt".to_owned());
        map.insert(26, "key.left.bracket".to_owned());
        map.insert(29, "key.left.control".to_owned());
        map.insert(42, "key.left.shift".to_owned());
        map.insert(219, "key.left.win".to_owned());
        map.insert(50, "key.m".to_owned());
        map.insert(12, "key.minus".to_owned());
        map.insert(49, "key.n".to_owned());
        map.insert(69, "key.num.lock".to_owned());
        map.insert(24, "key.o".to_owned());
        map.insert(25, "key.p".to_owned());
        map.insert(209, "key.page.down".to_owned());
        map.insert(201, "key.page.up".to_owned());
        map.insert(197, "key.pause".to_owned());
        map.insert(52, "key.period".to_owned());
        map.insert(183, "key.print.screen".to_owned());
        map.insert(16, "key.q".to_owned());
        map.insert(19, "key.r".to_owned());
        map.insert(205, "key.right".to_owned());
        map.insert(184, "key.right.alt".to_owned());
        map.insert(27, "key.right.bracket".to_owned());
        map.insert(157, "key.right.control".to_owned());
        map.insert(54, "key.right.shift".to_owned());
        map.insert(220, "key.right.win".to_owned());
        map.insert(31, "key.s".to_owned());
        map.insert(70, "key.scroll.lock".to_owned());
        map.insert(39, "key.semicolon".to_owned());
        map.insert(53, "key.slash".to_owned());
        map.insert(57, "key.space".to_owned());
        map.insert(20, "key.t".to_owned());
        map.insert(15, "key.tab".to_owned());
        map.insert(22, "key.u".to_owned());
        map.insert(200, "key.up".to_owned());
        map.insert(47, "key.v".to_owned());
        map.insert(17, "key.w".to_owned());
        map.insert(45, "key.x".to_owned());
        map.insert(21, "key.y".to_owned());
        map.insert(44, "key.z".to_owned());
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.options.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        for key in data.keys().cloned().collect::<Vec<_>>() {
            if !key.starts_with("key_") {
                continue;
            }
            let value = data.get_mut(&key).unwrap();
            let code = match value.as_string().and_then(|str| str.parse::<i32>().ok()) {
                Some(code) => code,
                None => continue
            };

            let new_entry = match code {
                -100 => "key.mouse.left".to_owned(),
                -99 => "key.mouse.right".to_owned(),
                -98 => "key.mouse.middle".to_owned(),
                i32::MIN..=-1 => format!("key.mouse.{}", code + 101),
                0..=255 => button_id_to_name().get(&(code as u8)).cloned().unwrap_or_else(|| "key.unknown".to_owned()),
                _ => "key.unknown".to_owned()
            };

            *value = T::Object::create_string(new_entry);
        }
    }));
}
