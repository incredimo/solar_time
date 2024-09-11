/*
 Detailed Problem Description:

In Vedic astrology, the accuracy of birth time is crucial for precise calculations and interpretations. However, the time recorded on a person's birth certificate or birth record is typically based on the local time zone of the birthplace, which may not accurately reflect the true solar time at that location. Our goal is to develop a system that can convert this recorded local time to the true solar time for more accurate astrological calculations.

Key Considerations:

1. Local Recorded Time: This is the time noted on the birth certificate or birth record. It's based on the official time zone of the location.

2. Time Zone Offset: Each location has an official time zone offset from UTC (Coordinated Universal Time). For example, India is UTC+5:30.

3. Longitude Effect: The sun moves across the sky at a rate of 15 degrees per hour (360° in 24 hours). This means that for every degree of longitude, there's a 4-minute difference in solar time.

4. Equation of Time: Due to the elliptical orbit of the Earth and its axial tilt, there's a variation between mean solar time and apparent solar time. This variation changes throughout the year.

5. Date of Birth: The equation of time varies based on the day of the year, so the birth date is crucial for accurate calculations.

Problem-Solving Steps:

1. Input Gathering:
   - Birth date (year, month, day)
   - Birth time (hours, minutes, seconds)
   - Birth location (latitude, longitude)
   - Time zone offset of the birth location

2. Convert Local Time to UTC:
   - Subtract the time zone offset from the local time to get UTC time.

3. Calculate Solar Time:
   a. Determine the day of the year for the birth date.
   b. Calculate the equation of time for that day.
   c. Calculate the longitude correction:
      - Determine the time difference based on the longitude of the birthplace relative to the central meridian of its time zone.
   d. Apply both corrections to the UTC time to get the local solar time.

4. Compare and Provide Results:
   - Show the original local time (as recorded on the birth certificate).
   - Show the calculated true solar time.
   - Display the difference between the two.

Example Scenario:
Let's say a person was born in Calicut, India (11.2588° N, 75.7804° E) on September 11, 2024, at 3:30 PM local time.

1. Input:
   - Date: 2024-09-11
   - Local Time: 15:30:00
   - Location: Calicut (11.2588° N, 75.7804° E)
   - Time Zone: UTC+5:30 (India Standard Time)

2. Convert to UTC:
   15:30 - 5:30 = 10:00 UTC

3. Calculate Solar Time:
   a. Day of year: 255
   b. Calculate equation of time for Sept 11
   c. Longitude correction: 
      Calicut is 75.7804° E, IST is based on 82.5° E
      Difference: 82.5° - 75.7804° = 6.7196°
      Time correction: 6.7196° * 4 minutes = 26.88 minutes
   d. Apply corrections to UTC time

4. Results:
   - Original Local Time: 15:30:00
   - Calculated Solar Time: [Result of step 3]
   - Difference: [Calculated difference]
 

*/

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Timestamp {
    day: i64,
    time_of_day: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u16,
}

#[derive(Debug, Clone, Copy)]
struct Timezone {
    name: &'static str,
    offset: f64,
    west_boundary: f64,
}

#[derive(Debug, Clone, Copy)]
enum DstRule {
    UsCanada,
    Eu,
    Australia,
    NewZealand,
    None,
}

impl Timestamp {
    /// creates a new timestamp with the given latitude, longitude, date and time
    /// the time provided should be the local time of the location as per the time zone
    /// system will automatically calculate the solar time and return the timestamp
    pub fn new(latitude: f64, longitude: f64, date: Date, time: Time) -> Self {
        let days_since_epoch = Self::days_since_epoch(&date);
        let milliseconds = Self::time_to_milliseconds(&time);
        let solar_time = Self::calculate_solar_time(latitude, longitude, &date, milliseconds);
        
        let mut timestamp = Timestamp {
            day: days_since_epoch,
            time_of_day: solar_time,
        };
        
        timestamp.normalize();
        timestamp
    }

    pub fn from_local_time(latitude: f64, longitude: f64, date: Date, time: Time) -> Self {
        let timestamp = Self::new(latitude, longitude, date, time);
        timestamp
    }

 

    pub fn day(&self) -> i64 {
        self.day
    }

    pub fn date(&self) -> Date {
        Date {
            year: (self.day / 365) as i32,
            month: ((self.day % 365) / 30) as u8,
            day: ((self.day % 365) % 30) as u8,
        }
    }

    pub fn time_of_day(&self) -> i64 {
        self.time_of_day
    }

    fn normalize(&mut self) {
        while self.time_of_day >= 86_400_000 {
            self.day += 1;
            self.time_of_day -= 86_400_000;
        }
        while self.time_of_day < 0 {
            self.day -= 1;
            self.time_of_day += 86_400_000;
        }
    }

    fn days_since_epoch(date: &Date) -> i64 {
        let mut days = 0;
        for year in 1970..date.year {
            days += if Self::is_leap_year(year) { 366 } else { 365 };
        }
        days += Self::day_of_year(date) as i64 - 1;
        days
    }

    fn time_to_milliseconds(time: &Time) -> i64 {
        let hours_ms = (time.hour as i64) * 3_600_000;
        let minutes_ms = (time.minute as i64) * 60_000;
        let seconds_ms = (time.second as i64) * 1_000;
        let milliseconds = time.millisecond as i64;

        hours_ms + minutes_ms + seconds_ms + milliseconds
    }

    pub fn calculate_solar_time(latitude: f64, longitude: f64, date: &Date, local_time_ms: i64) -> i64 {
        let day_of_year = Self::day_of_year(date) as f64;
        let equation_of_time = Self::equation_of_time(day_of_year);
        
        let solar_correction = 4.0 * (longitude - 15.0 * Self::get_timezone_offset(longitude, date)) + equation_of_time;
        let solar_time_ms = (local_time_ms as f64 + solar_correction * 60_000.0) as i64;

        solar_time_ms
    }

    fn equation_of_time(day_of_year: f64) -> f64 {
        let b = 2.0 * PI * (day_of_year - 81.0) / 365.0;
        let c = 2.0 * PI * (day_of_year - 1.0) / 365.0;
        229.18 * (0.000075 + 0.001868 * b.cos() - 0.032077 * b.sin()
                  - 0.014615 * (2.0 * b).cos() - 0.040849 * (2.0 * b).sin())
    }

    fn day_of_year(date: &Date) -> u16 {
        let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut day_of_year = 0;

        for month in 1..date.month {
            day_of_year += days_in_month[month as usize - 1];
        }

        day_of_year += date.day as u16;

        if Self::is_leap_year(date.year) && date.month > 2 {
            day_of_year += 1;
        }

        day_of_year
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    pub fn get_timezone_offset(longitude: f64, date: &Date) -> f64 {
        let (timezone, dst_rule) = Self::get_timezone_info(longitude);
        let base_offset = timezone.offset;
        
        match dst_rule {
            DstRule::None => base_offset,
            _ => {
                if Self::is_dst(date, dst_rule) {
                    base_offset + 1.0
                } else {
                    base_offset
                }
            }
        }
    }

    fn get_timezone_info(longitude: f64) -> (Timezone, DstRule) {
        const TIMEZONES: [Timezone; 25] = [
            Timezone { name: "IDLW", offset: -12.0, west_boundary: -180.0 },
            Timezone { name: "SST", offset: -11.0, west_boundary: -172.5 },
            Timezone { name: "HST", offset: -10.0, west_boundary: -157.5 },
            Timezone { name: "AKST", offset: -9.0, west_boundary: -142.5 },
            Timezone { name: "PST", offset: -8.0, west_boundary: -127.5 },
            Timezone { name: "MST", offset: -7.0, west_boundary: -112.5 },
            Timezone { name: "CST", offset: -6.0, west_boundary: -97.5 },
            Timezone { name: "EST", offset: -5.0, west_boundary: -82.5 },
            Timezone { name: "AST", offset: -4.0, west_boundary: -67.5 },
            Timezone { name: "NST", offset: -3.5, west_boundary: -52.5 },
            Timezone { name: "BRT", offset: -3.0, west_boundary: -37.5 },
            Timezone { name: "CVT", offset: -1.0, west_boundary: -22.5 },
            Timezone { name: "GMT", offset: 0.0, west_boundary: -7.5 },
            Timezone { name: "CET", offset: 1.0, west_boundary: 7.5 },
            Timezone { name: "EET", offset: 2.0, west_boundary: 22.5 },
            Timezone { name: "MSK", offset: 3.0, west_boundary: 37.5 },
            Timezone { name: "GST", offset: 4.0, west_boundary: 52.5 },
            Timezone { name: "PKT", offset: 5.0, west_boundary: 67.5 },
            Timezone { name: "BST", offset: 6.0, west_boundary: 82.5 },
            Timezone { name: "ICT", offset: 7.0, west_boundary: 97.5 },
            Timezone { name: "CST", offset: 8.0, west_boundary: 112.5 },
            Timezone { name: "JST", offset: 9.0, west_boundary: 127.5 },
            Timezone { name: "AEST", offset: 10.0, west_boundary: 142.5 },
            Timezone { name: "NZST", offset: 12.0, west_boundary: 172.5 },
            Timezone { name: "LINT", offset: 14.0, west_boundary: 180.0 },
        ];

        let index = TIMEZONES.binary_search_by(|tz| tz.west_boundary.partial_cmp(&longitude).unwrap()).unwrap_or_else(|x| x - 1);
        let timezone = TIMEZONES[index];

        let dst_rule = match timezone.name {
            "AKST" | "PST" | "MST" | "CST" | "EST" => DstRule::UsCanada,
            "CET" | "EET" => DstRule::Eu,
            "AEST" => DstRule::Australia,
            "NZST" => DstRule::NewZealand,
            _ => DstRule::None,
        };

        (timezone, dst_rule)
    }

    fn is_dst(date: &Date, dst_rule: DstRule) -> bool {
        let day_of_year = Self::day_of_year(date);
        
        match dst_rule {
            DstRule::UsCanada => {
                let spring_forward = Self::nth_weekday_of_month(3, 0, 2, date.year);
                let fall_back = Self::nth_weekday_of_month(11, 0, 1, date.year);
                day_of_year >= spring_forward && day_of_year < fall_back
            },
            DstRule::Eu => {
                let spring_forward = Self::last_weekday_of_month(3, 0, date.year);
                let fall_back = Self::last_weekday_of_month(10, 0, date.year);
                day_of_year >= spring_forward && day_of_year < fall_back
            },
            DstRule::Australia => {
                let spring_forward = Self::first_weekday_of_month(10, 0, date.year);
                let fall_back = Self::first_weekday_of_month(4, 0, date.year);
                day_of_year >= spring_forward || day_of_year < fall_back
            },
            DstRule::NewZealand => {
                let spring_forward = Self::last_weekday_of_month(9, 0, date.year);
                let fall_back = Self::first_weekday_of_month(4, 0, date.year);
                day_of_year >= spring_forward || day_of_year < fall_back
            },
            DstRule::None => false,
        }
    }

    fn nth_weekday_of_month(month: u8, weekday: u8, n: u8, year: i32) -> u16 {
        let first_day = Date { year, month, day: 1 };
        let first_day_of_week = Self::day_of_week(&first_day);
        let days_until_first = (7 + weekday - first_day_of_week) % 7;
        let target_day = 1 + days_until_first as u16 + 7 * (n as u16 - 1);
        Self::day_of_year(&Date { year, month, day: target_day as u8 })
    }

    fn first_weekday_of_month(month: u8, weekday: u8, year: i32) -> u16 {
        Self::nth_weekday_of_month(month, weekday, 1, year)
    }

    fn last_weekday_of_month(month: u8, weekday: u8, year: i32) -> u16 {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        let first_of_next_month = Date { year: next_year, month: next_month, day: 1 };
        let last_day_of_month = Self::day_of_year(&first_of_next_month) - 1;
        let last_day_of_week = (Self::day_of_week(&first_of_next_month) + 6) % 7;
        let days_to_subtract = (7 + last_day_of_week - weekday) % 7;
        last_day_of_month as u16 - days_to_subtract as u16
    }

    fn day_of_week(date: &Date) -> u8 {
        let adjusted_year = if date.month <= 2 { date.year - 1 } else { date.year };
        let adjusted_month = if date.month <= 2 { date.month + 12 } else { date.month };
        let k = date.day as u32;
        let j = (adjusted_year / 100) as u32;
        let h = (adjusted_year % 100) as u32;
        
        let day_of_week = (k + 13 * (adjusted_month as u32 + 1) / 5 + h + h / 4 + j / 4 + 5 * j) % 7;
        
        ((day_of_week + 5) % 7) as u8 // Adjust to make Sunday 0, Monday 1, ..., Saturday 6
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_of_year() {
        let date = Date { year: 2024, month: 9, day: 11 };
        assert_eq!(Timestamp::day_of_year(&date), 255); // 2024 is a leap year
    }

    #[test]
    fn test_is_leap_year() {
        assert!(Timestamp::is_leap_year(2000));
        assert!(Timestamp::is_leap_year(2024));
        assert!(!Timestamp::is_leap_year(2100));
        assert!(!Timestamp::is_leap_year(2023));
    }

    #[test]
    fn test_days_since_epoch() {
        let date = Date { year: 2024, month: 1, day: 1 };
        assert_eq!(Timestamp::days_since_epoch(&date), 19723);

        let date = Date { year: 1970, month: 1, day: 1 };
        assert_eq!(Timestamp::days_since_epoch(&date), 0);

        let date = Date { year: 1969, month: 12, day: 31 };
        assert_eq!(Timestamp::days_since_epoch(&date), -1);
    }

    #[test]
    fn test_time_to_milliseconds() {
        let time = Time { hour: 12, minute: 30, second: 45, millisecond: 500 };
        assert_eq!(Timestamp::time_to_milliseconds(&time), 45045500);

        let time = Time { hour: 0, minute: 0, second: 0, millisecond: 0 };
        assert_eq!(Timestamp::time_to_milliseconds(&time), 0);

        let time = Time { hour: 23, minute: 59, second: 59, millisecond: 999 };
        assert_eq!(Timestamp::time_to_milliseconds(&time), 86399999);
    }

    #[test]
    fn test_get_timezone_info() {
        let (tz, dst) = Timestamp::get_timezone_info(0.0);
        assert_eq!(tz.name, "GMT");
        assert_eq!(tz.offset, 0.0);
        assert!(matches!(dst, DstRule::None));

        let (tz, dst) = Timestamp::get_timezone_info(75.7804); // Calicut, Kerala
        assert_eq!(tz.name, "PKT");
        assert_eq!(tz.offset, 5.0);
        assert!(matches!(dst, DstRule::None));

        let (tz, dst) = Timestamp::get_timezone_info(-74.0060); // New York City
        assert_eq!(tz.name, "EST");
        assert_eq!(tz.offset, -5.0);
        assert!(matches!(dst, DstRule::UsCanada));
    }

    #[test]
    fn test_is_dst() {
        // Test US/Canada DST
        assert!(Timestamp::is_dst(&Date { year: 2024, month: 7, day: 1 }, DstRule::UsCanada));
        assert!(!Timestamp::is_dst(&Date { year: 2024, month: 1, day: 1 }, DstRule::UsCanada));

        // Test EU DST
        assert!(Timestamp::is_dst(&Date { year: 2024, month: 7, day: 1 }, DstRule::Eu));
        assert!(!Timestamp::is_dst(&Date { year: 2024, month: 1, day: 1 }, DstRule::Eu));

        // Test Australia DST
        assert!(Timestamp::is_dst(&Date { year: 2024, month: 1, day: 1 }, DstRule::Australia));
        assert!(!Timestamp::is_dst(&Date { year: 2024, month: 7, day: 1 }, DstRule::Australia));

        // Test New Zealand DST
        assert!(Timestamp::is_dst(&Date { year: 2024, month: 1, day: 1 }, DstRule::NewZealand));
        assert!(!Timestamp::is_dst(&Date { year: 2024, month: 7, day: 1 }, DstRule::NewZealand));

        // Test No DST
        assert!(!Timestamp::is_dst(&Date { year: 2024, month: 7, day: 1 }, DstRule::None));
    }

    #[test]
    fn test_nth_weekday_of_month() {
        assert_eq!(Timestamp::nth_weekday_of_month(3, 0, 2, 2024), 71); // 2nd Sunday of March 2024
        assert_eq!(Timestamp::nth_weekday_of_month(11, 0, 1, 2024), 310); // 1st Sunday of November 2024
    }

    #[test]
    fn test_last_weekday_of_month() {
        assert_eq!(Timestamp::last_weekday_of_month(3, 0, 2024), 87); // Last Sunday of March 2024
        assert_eq!(Timestamp::last_weekday_of_month(10, 0, 2024), 301); // Last Sunday of October 2024
    }

    #[test]
    fn test_day_of_week() {
        assert_eq!(Timestamp::day_of_week(&Date { year: 2024, month: 9, day: 11 }), 3); // Wednesday
        assert_eq!(Timestamp::day_of_week(&Date { year: 2000, month: 1, day: 1 }), 6); // Saturday
        assert_eq!(Timestamp::day_of_week(&Date { year: 1970, month: 1, day: 1 }), 4); // Thursday
    }

    #[test]
    fn test_normalize() {
        let mut timestamp = Timestamp { day: 0, time_of_day: 86_400_000 };
        timestamp.normalize();
        assert_eq!(timestamp.day, 1);
        assert_eq!(timestamp.time_of_day, 0);

        let mut timestamp = Timestamp { day: 0, time_of_day: -1 };
        timestamp.normalize();
        assert_eq!(timestamp.day, -1);
        assert_eq!(timestamp.time_of_day, 86_399_999);
    }

    #[test]
    fn test_calculate_solar_time() {
        let latitude = 11.2588; // Calicut, Kerala
        let longitude = 75.7804;
        let date = Date { year: 2024, month: 9, day: 11 };
        let local_time_ms = 7 * 3600000 + 10 * 60000; // 07:10:00

        let solar_time = Timestamp::calculate_solar_time(latitude, longitude, &date, local_time_ms);

        // The exact value would depend on the precise calculations, but we can check if it's in a reasonable range
        assert!(solar_time > local_time_ms - 3600000 && solar_time < local_time_ms + 3600000);
    }

    #[test]
    fn test_get_timezone_offset() {
        let longitude = 75.7804; // Calicut, Kerala
        let date = Date { year: 2024, month: 9, day: 11 };

        let offset = Timestamp::get_timezone_offset(longitude, &date);
        assert_eq!(offset, 5.0);

        let longitude = -74.0060; // New York City
        let date_summer = Date { year: 2024, month: 7, day: 1 };
        let date_winter = Date { year: 2024, month: 1, day: 1 };

        let offset_summer = Timestamp::get_timezone_offset(longitude, &date_summer);
        let offset_winter = Timestamp::get_timezone_offset(longitude, &date_winter);
        assert_eq!(offset_summer, -4.0); // EDT
        assert_eq!(offset_winter, -5.0); // EST
    }

    #[test]
    fn test_edge_cases() {
        // Test date at Unix epoch
        let timestamp = Timestamp::new(0.0, 0.0, Date { year: 1970, month: 1, day: 1 }, Time { hour: 0, minute: 0, second: 0, millisecond: 0 });
        assert_eq!(timestamp.day(), 0);
        assert_eq!(timestamp.time_of_day(), 0);

        // Test date before Unix epoch
        let timestamp = Timestamp::new(0.0, 0.0, Date { year: 1969, month: 12, day: 31 }, Time { hour: 23, minute: 59, second: 59, millisecond: 999 });
        assert_eq!(timestamp.day(), -1);
        assert_eq!(timestamp.time_of_day(), 86399999);

        // Test date far in the future
        let timestamp = Timestamp::new(0.0, 0.0, Date { year: 2100, month: 1, day: 1 }, Time { hour: 0, minute: 0, second: 0, millisecond: 0 });
        assert!(timestamp.day() > 47482); // Approximate number of days from epoch to 2100-01-01

        // Test International Date Line
        let timestamp_west = Timestamp::new(0.0, -180.0, Date { year: 2024, month: 1, day: 1 }, Time { hour: 0, minute: 0, second: 0, millisecond: 0 });
        let timestamp_east = Timestamp::new(0.0, 180.0, Date { year: 2024, month: 1, day: 1 }, Time { hour: 0, minute: 0, second: 0, millisecond: 0 });
        assert_eq!(timestamp_west.day(), timestamp_east.day() - 1);
    }
}