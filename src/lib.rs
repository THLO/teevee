pub struct Episode {
    title: String,
    season: u32,
    number: u32,
    total_number: u32,
}

impl Episode {
    pub fn new(title: String, season: u32, number: u32, total_number: u32) -> Episode {
        Episode{ title, season, number, total_number}
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn season(&self) -> u32 {
        self.season
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn total_number(&self) -> u32 {
        self.total_number
    }
}


pub struct TVShow {
    title: String,
    episodes: Vec<Episode>
}

impl TVShow {

    pub fn new(title: String) -> TVShow {
        TVShow{ title, episodes: Vec::new() }
    }
}

pub mod parsing {

    use chrono::{NaiveDate, Datelike, Duration};
    use std::error::Error;
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct DurationParseError;

    impl fmt::Display for DurationParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"{}", "Could not parse the provided interval.")
        }
    }

    impl Error for DurationParseError {
        fn description(&self) -> &str {
            "Could not parse the provided interval."
        }
    }

    fn parse_years_months_days(years_months_days : &str) -> Result<(i32, i32, i32), DurationParseError> {
        // Collect all characters but filtering out all spaces:
        let characters : Vec<char> = years_months_days.chars().filter(|c| !c.is_whitespace()).collect();
        let mut index = 0;
        let mut years : i32 = 0;
        let mut months : i32 = 0;
        let mut days : i32 = 0;
        while index < characters.len() {
            //  Read a number:
            let mut number : i32 = 0;
            while index < characters.len() && characters[index].is_digit(10) {
                number = number*10 +  (characters[index].to_digit(10).unwrap() as i32);
                index += 1;
            }
            // Read a time unit:
            let mut time_unit = String::new();
            while index < characters.len() && !characters[index].is_digit(10) {
                time_unit.push(characters[index]);
                index += 1;
            }
            // Match the time unit:
            if "years".starts_with(&time_unit) {
                years = number;
            }else if "months".starts_with(&time_unit) {
                months = ((number - 1) % 12) + 1;
            }else if "days".starts_with(&time_unit) {
                days = ((number - 1) % 31) + 1;
            }
        }
        if years != 0 || months != 0 || days != 0 {
                Ok((years, months, days))
        } else {
                Err(DurationParseError)
        }
    }

    pub fn parse_subtract_from_date(years_months_days : &str, date: NaiveDate) -> Result<NaiveDate, DurationParseError> {
        // Extract the number of years, months, and days, from the string slice:
        let (years, months, days) = parse_years_months_days(years_months_days)?;
        // Year after subtracting:
        let mut updated_year = (date.year() as i32) - years;
        // Month afeter subtracting;
        let mut updated_month = (date.month() as i32) - months;
        if updated_month <= 0 {
            updated_year -= 1;
            updated_month += 12;
        }
        // Create a new date based on the updated year and month:
        let updated_date = NaiveDate::from_ymd(updated_year, updated_month as u32, date.day());
        // Subtract the given number of days if any:
        if days > 0 {
            match updated_date.checked_sub_signed(Duration::days(days as i64)) {
                Some(date) => Ok(date),
                None => Err(DurationParseError)
            }
        }else {
            Ok(updated_date)
        }
    }
}

//fn read_number(characters: &Vec<char>, index: u32)

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
