use crate::MinecraftTypes;
use ahash::AHashMap;
use rust_dataconverter_engine::map_data_converter_func;
use std::sync::OnceLock;
use valence_nbt::Value;

const VERSION: u32 = 1344;

static BUTTON_ID_TO_NAME: OnceLock<AHashMap<u8, &'static str>> = OnceLock::new();

fn button_id_to_name() -> &'static AHashMap<u8, &'static str> {
    BUTTON_ID_TO_NAME.get_or_init(|| {
        let mut map = AHashMap::new();
        map.insert(0, "key.unknown");
        map.insert(11, "key.0");
        map.insert(2, "key.1");
        map.insert(3, "key.2");
        map.insert(4, "key.3");
        map.insert(5, "key.4");
        map.insert(6, "key.5");
        map.insert(7, "key.6");
        map.insert(8, "key.7");
        map.insert(9, "key.8");
        map.insert(10, "key.9");
        map.insert(30, "key.a");
        map.insert(40, "key.apostrophe");
        map.insert(48, "key.b");
        map.insert(43, "key.backslash");
        map.insert(14, "key.backspace");
        map.insert(46, "key.c");
        map.insert(58, "key.caps.lock");
        map.insert(51, "key.comma");
        map.insert(32, "key.d");
        map.insert(211, "key.delete");
        map.insert(208, "key.down");
        map.insert(18, "key.e");
        map.insert(207, "key.end");
        map.insert(28, "key.enter");
        map.insert(13, "key.equal");
        map.insert(1, "key.escape");
        map.insert(33, "key.f");
        map.insert(59, "key.f1");
        map.insert(68, "key.f10");
        map.insert(87, "key.f11");
        map.insert(88, "key.f12");
        map.insert(100, "key.f13");
        map.insert(101, "key.f14");
        map.insert(102, "key.f15");
        map.insert(103, "key.f16");
        map.insert(104, "key.f17");
        map.insert(105, "key.f18");
        map.insert(113, "key.f19");
        map.insert(60, "key.f2");
        map.insert(61, "key.f3");
        map.insert(62, "key.f4");
        map.insert(63, "key.f5");
        map.insert(64, "key.f6");
        map.insert(65, "key.f7");
        map.insert(66, "key.f8");
        map.insert(67, "key.f9");
        map.insert(34, "key.g");
        map.insert(41, "key.grave.accent");
        map.insert(35, "key.h");
        map.insert(199, "key.home");
        map.insert(23, "key.i");
        map.insert(210, "key.insert");
        map.insert(36, "key.j");
        map.insert(37, "key.k");
        map.insert(82, "key.keypad.0");
        map.insert(79, "key.keypad.1");
        map.insert(80, "key.keypad.2");
        map.insert(81, "key.keypad.3");
        map.insert(75, "key.keypad.4");
        map.insert(76, "key.keypad.5");
        map.insert(77, "key.keypad.6");
        map.insert(71, "key.keypad.7");
        map.insert(72, "key.keypad.8");
        map.insert(73, "key.keypad.9");
        map.insert(78, "key.keypad.add");
        map.insert(83, "key.keypad.decimal");
        map.insert(181, "key.keypad.divide");
        map.insert(156, "key.keypad.enter");
        map.insert(141, "key.keypad.equal");
        map.insert(55, "key.keypad.multiply");
        map.insert(74, "key.keypad.subtract");
        map.insert(38, "key.l");
        map.insert(203, "key.left");
        map.insert(56, "key.left.alt");
        map.insert(26, "key.left.bracket");
        map.insert(29, "key.left.control");
        map.insert(42, "key.left.shift");
        map.insert(219, "key.left.win");
        map.insert(50, "key.m");
        map.insert(12, "key.minus");
        map.insert(49, "key.n");
        map.insert(69, "key.num.lock");
        map.insert(24, "key.o");
        map.insert(25, "key.p");
        map.insert(209, "key.page.down");
        map.insert(201, "key.page.up");
        map.insert(197, "key.pause");
        map.insert(52, "key.period");
        map.insert(183, "key.print.screen");
        map.insert(16, "key.q");
        map.insert(19, "key.r");
        map.insert(205, "key.right");
        map.insert(184, "key.right.alt");
        map.insert(27, "key.right.bracket");
        map.insert(157, "key.right.control");
        map.insert(54, "key.right.shift");
        map.insert(220, "key.right.win");
        map.insert(31, "key.s");
        map.insert(70, "key.scroll.lock");
        map.insert(39, "key.semicolon");
        map.insert(53, "key.slash");
        map.insert(57, "key.space");
        map.insert(20, "key.t");
        map.insert(15, "key.tab");
        map.insert(22, "key.u");
        map.insert(200, "key.up");
        map.insert(47, "key.v");
        map.insert(17, "key.w");
        map.insert(45, "key.x");
        map.insert(21, "key.y");
        map.insert(44, "key.z");
        map
    })
}

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.options.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let mut replacements = Vec::new();
            for key in data.keys() {
                if !key.starts_with("key_") {
                    continue;
                }
                let Some(Value::String(code)) = data.get(key) else {
                    continue;
                };
                let Ok(code) = code.parse::<i32>() else {
                    continue;
                };

                let new_entry = match code {
                    -100 => "key.mouse.left".to_owned(),
                    -99 => "key.mouse.right".to_owned(),
                    -98 => "key.mouse.middle".to_owned(),
                    i32::MIN..=-1 => format!("key.mouse.{}", code + 101),
                    0..=255 => button_id_to_name()
                        .get(&(code as u8))
                        .copied()
                        .unwrap_or("key.unknown")
                        .to_owned(),
                    _ => "key.unknown".to_owned(),
                };

                replacements.push((key.clone(), new_entry));
            }

            for (key, new_entry) in replacements {
                data.insert(key, new_entry);
            }
        }),
    );
}
