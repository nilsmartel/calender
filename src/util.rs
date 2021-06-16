use crate::Weekday;

pub fn days_of_month(year: u16, month: u16) -> u16 {
    if month == 2 {
        return days_of_feb(year);
    }

    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 30
    }
}

fn is_leap_year(year: u16) -> bool {
    ((year % 4 == 0) && (year % 100!= 0)) || (year%400 == 0)
}

fn days_of_feb(year: u16) -> u16 {
    if is_leap_year(year) {
        29
    } else {
        28
    }
}

const MONTH_INFO: Vec<[Weekday; 12]> = Vec::new();

//
// returns the weekday of the first day of the given month
pub fn month_starts_with(year: u16, month: u16) -> Weekday {
    assert!(year >= 1970);
    assert!(month >= 1 && month < 13);

    // normalize inputs so we can use them to index into array
    let year_index = year as usize - 1970;
    let month_index = month as usize -1;

    {
        let mut month_info = MONTH_INFO;
        while month_info.len() <= year_index {
            generate_next_years_info(&mut month_info);
        }
    }

    MONTH_INFO[year_index][month_index]
}


fn generate_next_years_info(list: &mut Vec<[Weekday; 12]>) {
    // Note: Jan. 1st of 1970 was a thursday
    let start = Weekday::Thu;
    let year = list.len() as u16 + 1970;

    // First day of the year
    let mut first: u16 = if let Some(day) = list.last().map(|days| days.last().unwrap()) {
        day.next() as u16 -1
    } else {
        start as u16 -1
    };

    let mut days = [Weekday::Mo; 12];

    for i in 0..12 {
        days[i] = (first+1).into();

        let month = i as u16 + 1;
        first += days_of_month(year, month) % 7;
    }
}
