//******************************************************************************************************
//  ticks.rs - Gbtc
//
//  Copyright © 2023, Grid Protection Alliance.  All Rights Reserved.
//
//  Licensed to the Grid Protection Alliance (GPA) under one or more contributor license agreements. See
//  the NOTICE file distributed with this work for additional information regarding copyright ownership.
//  The GPA licenses this file to you under the MIT License (MIT), the "License"; you may not use this
//  file except in compliance with the License. You may obtain a copy of the License at:
//
//      http://opensource.org/licenses/MIT
//
//  Unless agreed to in writing, the subject software distributed under the License is distributed on an
//  "AS-IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. Refer to the
//  License for the specific language governing permissions and limitations.
//
//  Code Modification History:
//  ----------------------------------------------------------------------------------------------------
//  03/28/2023 - J. Ritchie Carroll
//       Generated original version of source code.
//
//******************************************************************************************************

use chrono::{DateTime, Local, TimeZone, Utc};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, Div, Mul, Sub};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Represents a 64-bit integer used to designate time in STTP. The value represents the number of 100-nanosecond
/// intervals that have elapsed since 12:00:00 midnight, January 1, 0001 UTC, Gregorian calendar.
///
/// A single tick represents one hundred nanoseconds, or one ten-millionth of a second. There are 10,000 ticks in
/// a millisecond and 10 million ticks in a second. Only bits 01 to 62 (0x3FFFFFFFFFFFFFFF) are used to represent
/// the timestamp value. Bit 64 (0x8000000000000000) is used to denote leap second, i.e., second 60, where actual
/// second value would remain at 59. Bit 63 (0x4000000000000000) is used to denote leap second direction,
/// 0 for add, 1 for delete.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ticks {
    val: u64,
}

impl Ticks {
    /// Number of `Ticks` that occur in a second.
    pub const PER_SECOND: u64 = 10_000_000;

    /// Number of `Ticks` that occur in a millisecond.
    pub const PER_MILLISECOND: u64 = Self::PER_SECOND;

    /// Number of `Ticks` that occur in a microsecond.
    pub const PER_MICROSECOND: u64 = Self::PER_SECOND / 1_000_000;

    /// Number of `Ticks` that occur in a minute.
    pub const PER_MINUTE: u64 = 60 * Self::PER_SECOND;

    /// Number of `Ticks` that occur in an hour.
    pub const PER_HOUR: u64 = 60 * Self::PER_MINUTE;

    /// Number of `Ticks` that occur in a day.
    pub const PER_DAY: u64 = 24 * Self::PER_HOUR;

    /// Flag (64th bit) that marks a `Ticks` value as a leap second, i.e., second 60 (one beyond normal second 59).
    pub const LEAP_SECOND_FLAG: u64 = 1 << 63;

    /// Flag (63rd bit) that indicates if leap second is positive or negative; 0 for add, 1 for delete.
    pub const LEAP_SECOND_DIRECTION: u64 = 1 << 62;

    /// All bits (bits 1 to 62) that make up the value portion of a `Ticks` that represent time.
    pub const VALUE_MASK: u64 = !Self::LEAP_SECOND_FLAG & !Self::LEAP_SECOND_DIRECTION;

    /// `Ticks` representation of the Unix epoch timestamp starting at January 1, 1970.
    pub const UNIX_BASE_OFFSET: u64 = 621_355_968_000_000_000;

    /// Creates a `Ticks` value.
    pub fn new(val: u64) -> Self {
        Self { val }
    }

    /// Gets the timestamp portion of the `Ticks` value, i.e.,
    /// the 62-bit time value excluding any leap second flags.
    pub fn timestamp_value(&self) -> u64 {
        self.val & Self::VALUE_MASK
    }

    /// Converts a standard Rust `DateTime` value to a `Ticks` value.
    pub fn from_datetime<Tz: TimeZone>(dt: DateTime<Tz>) -> Self
    where
        DateTime<Tz>: From<SystemTime>,
    {
        let duration = dt
            .signed_duration_since::<Tz>(UNIX_EPOCH.into())
            .to_std()
            .unwrap_or_default();

        Self {
            val: Self::from_duration(duration).val + Self::UNIX_BASE_OFFSET,
        }
    }

    /// Converts a standard Rust `Duration` value to a `Ticks` value.
    pub fn from_duration(duration: Duration) -> Self {
        Self {
            val: duration.as_secs() * Self::PER_SECOND + (duration.subsec_nanos() as u64) / 100,
        }
    }

    /// Converts a `Ticks` value to standard Rust `DateTime` value.
    pub fn to_datetime(&self) -> DateTime<Utc> {
        let value = self.timestamp_value() - Self::UNIX_BASE_OFFSET;
        let secs = value / Self::PER_SECOND;
        let nanos = (value % Self::PER_SECOND) * 100;
        let duration = Duration::new(secs, nanos as u32);
        DateTime::<Utc>::from(UNIX_EPOCH + duration)
    }

    /// Determines if the deserialized `Ticks` value represents a leap second, i.e., second 60.
    pub fn is_leap_second(&self) -> bool {
        (self.val & Self::LEAP_SECOND_FLAG) > 0
    }

    /// Returns a copy of this `Ticks` value flagged to represent a leap second, i.e., second 60, before wire serialization.
    pub fn set_leap_second(&self) -> Self {
        Self {
            val: self.val | Self::LEAP_SECOND_FLAG,
        }
    }

    /// Updates this `Ticks` value to represent a leap second, i.e., second 60, before wire serialization.
    pub fn apply_leap_second(&mut self) {
        self.val |= Self::LEAP_SECOND_FLAG;
    }

    /// Determines if the deserialized `Ticks` value represents a negative leap second, i.e., checks flag on second 58 to see if second 59 will be missing.
    pub fn is_negative_leap_second(&self) -> bool {
        self.is_leap_second() && (self.val & Self::LEAP_SECOND_DIRECTION) > 0
    }

    /// Returns a copy of this `Ticks` value flagged to represent a negative leap second, i.e., sets flag on second 58 to mark that second 59 will be missing, before wire serialization.
    pub fn set_negative_leap_second(&self) -> Self {
        Self {
            val: self.val | Self::LEAP_SECOND_FLAG | Self::LEAP_SECOND_DIRECTION,
        }
    }

    /// Updates this `Ticks` value to represent a negative leap second, i.e., sets flag on second 58 to mark that second 59 will be missing, before wire serialization.
    pub fn apply_negative_leap_second(&mut self) {
        self.val |= Self::LEAP_SECOND_FLAG | Self::LEAP_SECOND_DIRECTION;
    }

    /// Gets the current local time as a `Ticks` value.
    pub fn now() -> Self {
        let now: DateTime<Local> = Local::now();
        Self::from_datetime(now)
    }

    /// Gets the current time in UTC as a `Ticks` value.
    pub fn utc_now() -> Self {
        let now: DateTime<Utc> = Utc::now();
        Self::from_datetime(now)
    }

    /// Standard timestamp representation for a `Ticks` value, e.g., 2006-01-02 15:04:05.999999999.
    pub fn to_string(&self) -> String {
        let result = self
            .to_datetime()
            .to_rfc3339_opts(chrono::SecondsFormat::Nanos, true)
            .replace("T", " ");
        result[..result.len() - 1].to_string()
    }

    /// Shows just the timestamp portion of a `Ticks` value with milliseconds, e.g., 15:04:05.999.
    pub fn to_short_string(&self) -> String {
        let datetime_str = self.to_string();
        let result = datetime_str.split(" ").nth(1).unwrap_or("").to_string();
        result[..result.len() - 6].to_string()
    }
}

impl Display for Ticks {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_string())
    }
}

impl Add for Ticks {
    type Output = Ticks;

    fn add(self, rhs: Ticks) -> Self {
        Ticks {
            val: self.val + rhs.val,
        }
    }
}

impl Sub for Ticks {
    type Output = Ticks;

    fn sub(self, rhs: Ticks) -> Self {
        Ticks {
            val: self.val - rhs.val,
        }
    }
}

impl Mul for Ticks {
    type Output = Ticks;

    fn mul(self, rhs: Ticks) -> Self {
        Ticks {
            val: self.val * rhs.val,
        }
    }
}

impl Div for Ticks {
    type Output = Ticks;

    fn div(self, rhs: Ticks) -> Self {
        Ticks {
            val: self.val / rhs.val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ticks;
    use chrono::{DateTime, TimeZone, Timelike, Utc};
    use lazy_static::lazy_static;

    const TEST_TICK_VAL: u64 = 637669683993391278;

    lazy_static! {
        static ref TEST_DATETIME: DateTime<Utc> = init_test_datetime();
        static ref TEST_TICKS: Ticks = Ticks { val: TEST_TICK_VAL };
        static ref TEST_DATETIME_VAL: String = "2021-09-11 14:46:39.339127800".to_string();
    }

    fn init_test_datetime() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2021, 9, 11, 14, 46, 39)
            .unwrap()
            .with_nanosecond(339127800)
            .unwrap()
    }

    #[test]
    fn test_ticks_from_datetime() {
        let ticks = Ticks::from_datetime(*TEST_DATETIME);

        assert_eq!(ticks.val, TEST_TICK_VAL);
    }

    #[test]
    fn test_ticks_to_datetime() {
        let dt = TEST_TICKS.to_datetime();

        assert_eq!(dt, *TEST_DATETIME);
    }

    #[test]
    fn test_ticks_is_leap_second() {
        let ticks = *TEST_TICKS;
        let leap_second_ticks = ticks.set_leap_second();

        assert!(!ticks.is_leap_second());
        assert!(leap_second_ticks.is_leap_second());
    }

    #[test]
    fn test_ticks_set_leap_second() {
        let ticks = *TEST_TICKS;
        let leap_second_ticks = ticks.set_leap_second();

        assert_eq!(leap_second_ticks.val, ticks.val | Ticks::LEAP_SECOND_FLAG);
    }

    #[test]
    fn test_ticks_is_negative_leap_second() {
        let ticks = *TEST_TICKS;
        let negative_leap_second_ticks = ticks.set_negative_leap_second();

        assert!(!ticks.is_negative_leap_second());
        assert!(negative_leap_second_ticks.is_negative_leap_second());
    }

    #[test]
    fn test_ticks_set_negative_leap_second() {
        let ticks = *TEST_TICKS;
        let negative_leap_second_ticks = ticks.set_negative_leap_second();

        assert_eq!(
            negative_leap_second_ticks.val,
            ticks.val | Ticks::LEAP_SECOND_FLAG | Ticks::LEAP_SECOND_DIRECTION
        );
    }

    #[test]
    fn test_ticks_to_string() {
        let ticks = *TEST_TICKS;
        let string_representation = ticks.to_string();

        assert_eq!(string_representation, *TEST_DATETIME_VAL);
    }

    #[test]
    fn test_ticks_to_short_string() {
        let ticks = *TEST_TICKS;
        let short_string_representation = ticks.to_short_string();

        assert_eq!(short_string_representation, "14:46:39.339");
    }
}
