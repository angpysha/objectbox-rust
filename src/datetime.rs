/// DateTime types for ObjectBox, analogous to Dart's PropertyType.date / dateNano / dateUtc / dateNanoUtc.
///
/// ObjectBox stores dates as 64-bit integers:
/// - `OBXPropertyType_Date` (10): milliseconds since Unix epoch (1970-01-01 00:00:00 UTC)
/// - `OBXPropertyType_DateNano` (12): nanoseconds since Unix epoch
///
/// # Variants
///
/// | Rust Type       | Dart Equivalent            | OBX Type | Precision    | UTC |
/// |-----------------|----------------------------|----------|--------------|-----|
/// | `DateTime`      | `PropertyType.dateUtc`     | Date(10) | milliseconds | yes |
/// | `DateTimeNano`  | `PropertyType.dateNanoUtc` | DateNano(12) | nanoseconds | yes |
///
/// # Examples
///
/// ```rust
/// use objectbox::datetime::{DateTime, DateTimeNano};
///
/// // Create from milliseconds since epoch
/// let dt = DateTime::from_millis(1706745600000); // 2024-02-01 00:00:00 UTC
///
/// // Create from current time
/// let now = DateTime::now();
///
/// // Access the raw value
/// let ms: i64 = now.to_millis();
///
/// // Nanosecond precision
/// let dt_nano = DateTimeNano::now();
/// let ns: i64 = dt_nano.to_nanos();
/// ```

use std::time::{SystemTime, UNIX_EPOCH};

/// A UTC datetime stored as milliseconds since Unix epoch.
///
/// Corresponds to Dart's `PropertyType.dateUtc` and ObjectBox `OBXPropertyType_Date` (10).
/// This is the recommended type for most datetime fields.
///
/// # Storage
/// Stored as `i64` in the database. Value of `0` represents the Unix epoch (1970-01-01 00:00:00 UTC).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DateTime(pub i64);

/// A UTC datetime stored as nanoseconds since Unix epoch.
///
/// Corresponds to Dart's `PropertyType.dateNanoUtc` and ObjectBox `OBXPropertyType_DateNano` (12).
/// Use this when you need nanosecond precision.
///
/// # Storage
/// Stored as `i64` in the database. Value of `0` represents the Unix epoch (1970-01-01 00:00:00 UTC).
///
/// # Note
/// Rust's `SystemTime` has nanosecond precision on most platforms.
/// Dart's `DateTime` only supports microsecond precision, so Dart uses
/// `microsecondsSinceEpoch * 1000` for nanosecond approximation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DateTimeNano(pub i64);

// ==================== DateTime (millisecond precision) ====================

impl DateTime {
    /// Create a DateTime from milliseconds since Unix epoch.
    pub fn from_millis(ms: i64) -> Self {
        DateTime(ms)
    }

    /// Get the milliseconds since Unix epoch.
    pub fn to_millis(self) -> i64 {
        self.0
    }

    /// Create a DateTime representing the current UTC time.
    pub fn now() -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        DateTime(duration.as_millis() as i64)
    }

    /// Create a DateTime from seconds since Unix epoch.
    pub fn from_secs(secs: i64) -> Self {
        DateTime(secs * 1000)
    }

    /// Get seconds since Unix epoch (truncating milliseconds).
    pub fn to_secs(self) -> i64 {
        self.0 / 1000
    }

    /// Returns true if this datetime represents the zero/default value (epoch).
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<i64> for DateTime {
    fn from(ms: i64) -> Self {
        DateTime(ms)
    }
}

impl From<DateTime> for i64 {
    fn from(dt: DateTime) -> Self {
        dt.0
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = self.0 / 1000;
        let ms = (self.0 % 1000).abs();
        write!(f, "DateTime({}s {}ms)", secs, ms)
    }
}

// ==================== DateTimeNano (nanosecond precision) ====================

impl DateTimeNano {
    /// Create a DateTimeNano from nanoseconds since Unix epoch.
    pub fn from_nanos(ns: i64) -> Self {
        DateTimeNano(ns)
    }

    /// Get the nanoseconds since Unix epoch.
    pub fn to_nanos(self) -> i64 {
        self.0
    }

    /// Create a DateTimeNano representing the current UTC time.
    pub fn now() -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        DateTimeNano(duration.as_nanos() as i64)
    }

    /// Create a DateTimeNano from milliseconds since Unix epoch.
    pub fn from_millis(ms: i64) -> Self {
        DateTimeNano(ms * 1_000_000)
    }

    /// Get milliseconds since Unix epoch (truncating nanoseconds).
    pub fn to_millis(self) -> i64 {
        self.0 / 1_000_000
    }

    /// Create a DateTimeNano from microseconds since Unix epoch
    /// (compatible with Dart's `DateTime.microsecondsSinceEpoch`).
    pub fn from_micros(us: i64) -> Self {
        DateTimeNano(us * 1000)
    }

    /// Get microseconds since Unix epoch (truncating nanoseconds).
    pub fn to_micros(self) -> i64 {
        self.0 / 1000
    }

    /// Returns true if this datetime represents the zero/default value (epoch).
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Convert to a millisecond-precision DateTime (truncating nanoseconds).
    pub fn to_datetime(self) -> DateTime {
        DateTime(self.to_millis())
    }
}

impl From<i64> for DateTimeNano {
    fn from(ns: i64) -> Self {
        DateTimeNano(ns)
    }
}

impl From<DateTimeNano> for i64 {
    fn from(dt: DateTimeNano) -> Self {
        dt.0
    }
}

impl From<DateTime> for DateTimeNano {
    fn from(dt: DateTime) -> Self {
        DateTimeNano::from_millis(dt.to_millis())
    }
}

impl std::fmt::Display for DateTimeNano {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = self.0 / 1_000_000_000;
        let ns = (self.0 % 1_000_000_000).abs();
        write!(f, "DateTimeNano({}s {}ns)", secs, ns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_from_millis() {
        let dt = DateTime::from_millis(1706745600000);
        assert_eq!(dt.to_millis(), 1706745600000);
        assert_eq!(dt.to_secs(), 1706745600);
    }

    #[test]
    fn test_datetime_from_secs() {
        let dt = DateTime::from_secs(1706745600);
        assert_eq!(dt.to_millis(), 1706745600000);
    }

    #[test]
    fn test_datetime_now() {
        let dt = DateTime::now();
        assert!(dt.to_millis() > 0);
    }

    #[test]
    fn test_datetime_default() {
        let dt = DateTime::default();
        assert_eq!(dt.to_millis(), 0);
        assert!(dt.is_zero());
    }

    #[test]
    fn test_datetime_conversions() {
        let ms: i64 = 1706745600000;
        let dt: DateTime = ms.into();
        let back: i64 = dt.into();
        assert_eq!(ms, back);
    }

    #[test]
    fn test_datetime_nano_from_nanos() {
        let dt = DateTimeNano::from_nanos(1706745600_000_000_000);
        assert_eq!(dt.to_nanos(), 1706745600_000_000_000);
        assert_eq!(dt.to_millis(), 1706745600_000);
        assert_eq!(dt.to_micros(), 1706745600_000_000);
    }

    #[test]
    fn test_datetime_nano_from_millis() {
        let dt = DateTimeNano::from_millis(1706745600000);
        assert_eq!(dt.to_nanos(), 1706745600_000_000_000);
    }

    #[test]
    fn test_datetime_nano_from_micros() {
        // Compatible with Dart's DateTime.microsecondsSinceEpoch
        let dt = DateTimeNano::from_micros(1706745600_000_000);
        assert_eq!(dt.to_nanos(), 1706745600_000_000_000);
    }

    #[test]
    fn test_datetime_nano_to_datetime() {
        let nano = DateTimeNano::from_nanos(1706745600_123_456_789);
        let dt = nano.to_datetime();
        assert_eq!(dt.to_millis(), 1706745600_123); // nanoseconds truncated
    }

    #[test]
    fn test_datetime_to_datetime_nano() {
        let dt = DateTime::from_millis(1706745600_123);
        let nano: DateTimeNano = dt.into();
        assert_eq!(nano.to_nanos(), 1706745600_123_000_000);
    }

    #[test]
    fn test_datetime_nano_now() {
        let dt = DateTimeNano::now();
        assert!(dt.to_nanos() > 0);
    }
}
