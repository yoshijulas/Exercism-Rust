// black - 0
// brown - 1
// red - 2
// orange - 3
// yellow - 4
// green - 5
// blue - 6
// violet - 7
// grey - 8
// white - 9

use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    ResistorColor::from_int(value).map_or_else(
        |_| "value out of range".to_string(),
        |color| format!("{color:?}"),
    )
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors = all::<ResistorColor>().collect::<Vec<ResistorColor>>();
    colors.sort_by_key(|a| a.int_value());
    colors
}
