#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    fn new(year: u16, month: u8, day: u8) -> Date {
        Date { year, month, day }
    }

    fn num_days_this_month(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!("Invalid month"),
        }
    }

    fn next(&self) -> Date {
        if self.day == self.num_days_this_month() {
            if self.month == 12 {
                Date::new(self.year + 1, 1, 1)
            } else {
                Date::new(self.year, self.month + 1, 1)
            }
        } else {
            Date::new(self.year, self.month, self.day + 1)
        }
    }
}

fn main() {
    let mut date = Date::new(1900, 1, 1);
    let mut weekday = 1;
    let start = Date::new(1901, 1, 1);
    let end = Date::new(2000, 12, 31);
    let mut result = 0;
    while date <= end {
        if date >= start && date.day == 1 && weekday == 0 {
            result += 1;
        }
        date = date.next();
        weekday = (weekday + 1) % 7;
    }
    println!("{}", result);
}
