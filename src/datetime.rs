use crate::*;

const SECONDS_PER_DAY: i64 = 100000;
const DAYS_PER_MONTH: i64 = 30;
const DAYS_PER_DECADE: i64 = 10;
const SECONDS_PER_MONTH: i64 = SECONDS_PER_DAY*DAYS_PER_MONTH;

pub struct DateTime {
    year0: i64,
    month0: i64,
    day0: i64,
    hour: i64,
    minute: i64,
    second: i64,
}

impl DateTime {
    pub fn from_timestamp(timestamp: Timestamp) -> Self {
        let year0 = ts_to_year0(timestamp.seconds);
        let seconds_in_year = timestamp.seconds - get_year_start0(year0);

        let month0 = seconds_in_year.div_euclid(SECONDS_PER_MONTH);
        let seconds_in_month = seconds_in_year.rem_euclid(SECONDS_PER_MONTH);

        let day0 = seconds_in_month.div_euclid(SECONDS_PER_DAY);
        let seconds_in_day = seconds_in_month.rem_euclid(SECONDS_PER_DAY);

        let hour = seconds_in_day.div_euclid(10000);
        let seconds_in_hour = seconds_in_day.rem_euclid(1000);

        let minute = seconds_in_hour.div_euclid(100);
        let second = seconds_in_hour.rem_euclid(100);

        Self {
            year0,
            month0,
            day0,
            hour,
            minute,
            second,
        }
    }

    pub fn from_ymd_hms(year: i64, month: i64, day: i64, hour: i64, minute: i64, second: i64) -> Self {
        let year0 = match year > 0 {
            true => year - 1,
            false => year
        };
        let month0 = month - 1;
        let day0 = day - 1;
        Self {
            year0,
            month0,
            day0,
            hour,
            minute,
            second,
        }
    }

    pub fn from_ymd_hms0(year0: i64, month0: i64, day0: i64, hour: i64, minute: i64, second: i64) -> Self {
        Self {
            year0,
            month0,
            day0,
            hour,
            minute,
            second,
        }
    }

    pub fn from_ymd(year: i64, month: i64, day: i64) -> Self {
        Self::from_ymd_hms(year, month, day, 0, 0, 0)
    }

    pub fn from_ymd0(year0: i64, month0: i64, day0: i64) -> Self {
        Self::from_ymd_hms0(year0, month0, day0, 0, 0, 0)
    }

    /// Returns the franciade but starting from 0.
    /// A franciade is a period of 4 years.
    pub fn franciade0(&self) -> i64 {
        ((self.year0 + 2) / 4) - 1
    }

    /// Returns the franciade but starting from 1.
    /// There is no franciade 0.
    pub fn franciade(&self) -> i64 {
        let franciade0 = self.franciade0();
        match franciade0 > 0 {
            true => franciade0 + 1,
            false => franciade0,
        }
    }

    /// Returns the year but starting from 0.
    pub fn year0(&self) -> i64 {
        self.year0
    }

    /// Returns the year but starting from 1.
    pub fn year(&self) -> i64 {
        match self.year0 > 0 {
            true => self.year0 + 1,
            false => self.year0,
        }
    }

    /// Returns the month but starting from 0.
    /// A 13th month of 5 or 6 days is added at the end of the year.
    pub fn month0(&self) -> i64 {
        self.month0
    }

    /// Returns the month name, starting with a capital letter.
    pub fn month_name(&self) -> &'static str {
        let month0 = self.month0();
        match month0 {
            0 => "Vendémiaire",
            1 => "Brumaire",
            2 => "Frimaire",
            3 => "Nivôse",
            4 => "Pluviôse",
            5 => "Ventôse",
            6 => "Germinal",
            7 => "Floréal",
            8 => "Prairial",
            9 => "Messidor",
            10 => "Thermidor",
            11 => "Fructidor",
            12 => "Sansculotides",
            _ => unreachable!(),
        }
    }

    /// Returns the month name, all lowercase.
    pub fn month_name_lc(&self) -> &'static str {
        let month0 = self.month0();
        match month0 {
            0 => "vendémiaire",
            1 => "brumaire",
            2 => "frimaire",
            3 => "nivôse",
            4 => "pluviôse",
            5 => "ventôse",
            6 => "germinal",
            7 => "floréal",
            8 => "prairial",
            9 => "messidor",
            10 => "thermidor",
            11 => "fructidor",
            12 => "sansculotides",
            _ => unreachable!(),
        }
    }

    /// Returns the month, starting from 1.
    /// A 13th month of 5 or 6 days is added at the end of the year.
    pub fn month(&self) -> i64 {
        self.month0() + 1
    }

    /// Returns the day of the month but starting from 0.
    pub fn day0(&self) -> i64 {
        self.day0
    }

    /// Returns the day of the month, starting from 1.
    pub fn day(&self) -> i64 {
        self.day0() + 1
    }

    /// Returns the decade but starting from 0.
    pub fn decade0(&self) -> i64 {
        self.day0().div_euclid(DAYS_PER_DECADE)
    }

    /// Returns the decade, starting from 1.
    pub fn decade(&self) -> i64 {
        self.decade0() + 1
    }

    /// Returns the day of the week but starting from 0.
    pub fn weekday0(&self) -> i64 {
        self.day0().rem_euclid(DAYS_PER_DECADE)
    }

    /// Returns the day of the week, starting from 1.
    pub fn weekday(&self) -> i64 {
        self.weekday0() + 1
    }

    /// Returns the day of the week name, starting with a capital letter.
    pub fn weekday_name(&self) -> &'static str {
        if self.month0() == 12 {
            return match self.day0() {
                0 => "Jour de la vertu",
                1 => "Jour du génie",
                2 => "Jour du travail",
                3 => "Jour de l'opinion",
                4 => "Jour des récompenses",
                5 => "Jour de la Révolution",
                _ => unreachable!(),
            }
        }
        let weekday0 = self.weekday0();
        match weekday0 {
            0 => "Primidi",
            1 => "Duodi",
            2 => "Tridi",
            3 => "Quartidi",
            4 => "Quintidi",
            5 => "Sextidi",
            6 => "Septidi",
            7 => "Octidi",
            8 => "Nonidi",
            9 => "Décadi",
            _ => unreachable!(),
        }
    }

    /// Returns the day of the week name, all lowercase (except Révolution)
    pub fn weekday_name_lc(&self) -> &'static str {
        if self.month0() == 12 {
            return match self.day0() {
                0 => "jour de la vertu",
                1 => "jour du génie",
                2 => "jour du travail",
                3 => "jour de l'opinion",
                4 => "jour des récompenses",
                5 => "jour de la Révolution",
                _ => unreachable!(),
            }
        }
        let weekday0 = self.weekday0();
        match weekday0 {
            0 => "primidi",
            1 => "duodi",
            2 => "tridi",
            3 => "quartidi",
            4 => "quintidi",
            5 => "sextidi",
            6 => "septidi",
            7 => "octidi",
            8 => "nonidi",
            9 => "décadi",
            _ => unreachable!(),
        }
    }

    pub fn hour(&self) -> i64 {
        self.hour
    }

    pub fn minute(&self) -> i64 {
        self.minute
    }

    pub fn second(&self) -> i64 {
        self.second
    }

    pub fn hms(&self) -> (i64, i64, i64) {
        (self.hour, self.minute, self.second)
    }

    fn fmt_default(&self, f: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(
            f,
            "{} {} {} {}",
            self.weekday_name(),
            self.day(),
            self.month_name(),
            self.year(),
        )
    }

    fn fmt_traditional(&self, f: &mut impl std::io::Write) -> std::io::Result<()> {
        let mut remaining_years = self.year();
        let thousand_years = remaining_years.div_euclid(1000);
        remaining_years -= thousand_years * 1000;
        let thousand_years = "M".repeat(thousand_years as usize);
        let five_hundred_years = remaining_years.div_euclid(500);
        remaining_years -= five_hundred_years * 500;
        let five_hundred_years = match five_hundred_years {
            0 => "",
            1 => "D",
            _ => unreachable!(),
        };
        let hundred_years = remaining_years.div_euclid(100);
        remaining_years -= hundred_years * 100;
        let hundred_years = match hundred_years {
            0 => "",
            1 => "C",
            2 => "CC",
            3 => "CCC",
            4 => "CD",
            _ => unreachable!(),
        };
        let fifty_years = remaining_years.div_euclid(50);
        remaining_years -= fifty_years * 50;
        let fifty_years = match fifty_years {
            0 => "",
            1 => "L",
            _ => unreachable!(),
        };
        let ten_years = remaining_years.div_euclid(10);
        remaining_years -= ten_years * 10;
        let ten_years = match ten_years {
            0 => "",
            1 => "X",
            2 => "XX",
            3 => "XXX",
            4 => "XL",
            _ => unreachable!(),
        };
        let five_years = remaining_years.div_euclid(5);
        remaining_years -= five_years * 5;
        let five_years = match five_years {
            0 => "",
            1 => "V",
            _ => unreachable!(),
        };
        let one_year = remaining_years;
        let one_year = match one_year {
            0 => "",
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            _ => unreachable!(),
        };

        write!(
            f,
            "{} {} {} an {}{}{}{}{}{}{}",
            self.weekday_name(),
            self.day(),
            self.month_name(),
            thousand_years, five_hundred_years, hundred_years, fifty_years, ten_years, five_years, one_year
        )
    }

    pub fn to_string_default(&self) -> String {
        let mut s = Vec::new();
        self.fmt_default(&mut s).unwrap();
        String::from_utf8(s).unwrap()
    }

    pub fn to_string_traditional(&self) -> String {
        let mut s = Vec::new();
        self.fmt_traditional(&mut s).unwrap();
        String::from_utf8(s).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SECONDS_PER_YEAR: i64 = 365*SECONDS_PER_DAY;

    #[test]
    fn test_franciade() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.franciade0(), 0);
        assert_eq!(datetime.franciade(), 1);

        let datetime = DateTime::from_timestamp(Timestamp { seconds: -1 });
        assert_eq!(datetime.franciade0(), -1);
        assert_eq!(datetime.franciade(), -1);

        let datetime = DateTime::from_timestamp(Timestamp { seconds: SECONDS_PER_YEAR*5 });
        assert_eq!(datetime.franciade0(), 1);
        assert_eq!(datetime.franciade(), 2);
    }

    #[test]
    fn test_year() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.year0(), 0);
        assert_eq!(datetime.year(), 1);
        
        let datetime = DateTime::from_timestamp(Timestamp { seconds: -1 });
        assert_eq!(datetime.year0(), -1);
        assert_eq!(datetime.year(), -1);

        let datetime = DateTime::from_timestamp(Timestamp { seconds: -SECONDS_PER_YEAR-SECONDS_PER_DAY-1 });
        assert_eq!(datetime.year0(), -2);
        assert_eq!(datetime.year(), -2);
    }

    #[test]
    fn test_month() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.month0(), 0);
        assert_eq!(datetime.month(), 1);
        
        let datetime = DateTime::from_timestamp(Timestamp { seconds: -1 });
        assert_eq!(datetime.month0(), 12);
        assert_eq!(datetime.month(), 13);
    }

    #[test]
    fn test_day() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.day0(), 0);
        assert_eq!(datetime.day(), 1);
        
        let datetime = DateTime::from_timestamp(Timestamp { seconds: get_year_start(4)-1 });
        assert_eq!(datetime.day0(), 5); // Jour de la révolution
        assert_eq!(datetime.weekday_name(), "Jour de la Révolution");
        assert_eq!(datetime.day(), 6);
    }

    #[test]
    fn test_fmt() {
        let datetime = DateTime::from_timestamp(Timestamp { seconds: 0 });
        assert_eq!(datetime.to_string_default(), "Primidi 1 Vendémiaire 1");
        assert_eq!(datetime.to_string_traditional(), "Primidi 1 Vendémiaire an I");
    }
}
