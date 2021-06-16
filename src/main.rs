mod util;
use druid::*;
use druid::{Data, Lens };

fn main() {
    let window = WindowDesc::new(calender_cell);
    let data = CalenderState { year: 2021, month:6, day: 16 };

    AppLauncher::with_window(window).launch(data).unwrap();
}

#[derive(Clone, Copy, Lens, Data)]
struct CalenderState {
    year: u16,
    month: u16,
    day: u16,
}

#[derive(Copy, Clone, Debug)]
pub enum Weekday {
    Mo, Tue, Wed, Thu, Fr, Sa, Su
}

impl From<u16> for Weekday {
    fn from(i: u16) -> Self {
        use Weekday::*;
        match i {
            1 => Mo,
            2 => Tue,
            3 => Wed,
            4 => Thu,
            5 => Fr,
            6 => Sa,
            7 => Su,
            _ => panic!("not a valid weekday value, must be between 1 and 7")
        }
    }
}

impl Into<u16> for Weekday {
    fn into(self) -> u16 {
        self as u16 + 1
    }
}

impl From<u8> for Weekday {
    fn from(i: u8) -> Self {
        use Weekday::*;
        match i {
            1 => Mo,
            2 => Tue,
            3 => Wed,
            4 => Thu,
            5 => Fr,
            6 => Sa,
            7 => Su,
            _ => panic!("not a valid weekday value, must be between 1 and 7")
        }
    }
}

impl Into<u8> for Weekday {
    fn into(self) -> u8 {
        self as u8 + 1
    }
}


impl Weekday {
    fn to_str(self) -> &'static str {
        use Weekday::*;
        match self {
            Mo => "mo",
            Tue => "tue",
            Wed => "wed",
            Thu => "thu",
            Fr => "fr",
            Sa => "sa",
            Su => "su",
        }
    }

    fn next(self) -> Self {
        let value: u8 = self.into();
        ((value-1)%7 + 1).into()
    }
}

impl CalenderState {
    // TODO implement
    fn weekday(&self) -> Weekday { Weekday::Wed }
}

fn calender_cell() -> impl Widget<CalenderState> {
    widget::Flex::row()
        .with_child(widget::Label::new(|data: &CalenderState, env: &_| data.weekday().to_str().to_string()))
        .with_child(day_display())
}

fn day_display() -> impl Widget<CalenderState> {
    let font = FontDescriptor::new(FontFamily::new_unchecked("serif")).with_size(32.0);
    widget::Label::new(|data: &CalenderState, env: &_| format!("{}", data.day)).with_font(font)
}
