use std::fmt;
use std::hash::Hash;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ByteSize(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitSystem {
    Binary,
    Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    InvalidNumber,
    UnknownUnit,
    Overflow,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "input is empty"),
            ParseError::InvalidNumber => write!(f, "invalid numeric value"),
            ParseError::UnknownUnit => write!(f, "unknown byte unit"),
            ParseError::Overflow => write!(f, "value is too large"),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<u64> for ByteSize {
    fn from(value: u64) -> Self {
        ByteSize(value)
    }
}

impl From<ByteSize> for u64 {
    fn from(value: ByteSize) -> Self {
        value.0
    }
}

impl FromStr for ByteSize {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        ByteSize::parse(input)
    }
}

impl ByteSize {
    pub const ZERO: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn from_bytes(value: u64) -> Self {
        Self(value)
    }

    pub fn from_kilobytes(value: f64) -> Self {
        Self((value * 1_000.0).round().max(0.0) as u64)
    }

    pub fn from_megabytes(value: f64) -> Self {
        Self((value * 1_000_000.0).round().max(0.0) as u64)
    }

    pub fn from_gigabytes(value: f64) -> Self {
        Self((value * 1_000_000_000.0).round().max(0.0) as u64)
    }

    pub fn from_terabytes(value: f64) -> Self {
        Self((value * 1_000_000_000_000.0).round().max(0.0) as u64)
    }

    pub fn from_petabytes(value: f64) -> Self {
        Self((value * 1_000_000_000_000_000.0).round().max(0.0) as u64)
    }

    pub fn from_exabytes(value: f64) -> Self {
        Self((value * 1_000_000_000_000_000_000.0).round().max(0.0) as u64)
    }

    pub fn from_kibibytes(value: f64) -> Self {
        Self((value * 1024.0).round().max(0.0) as u64)
    }

    pub fn from_mebibytes(value: f64) -> Self {
        Self((value * 1_048_576.0).round().max(0.0) as u64)
    }

    pub fn from_gibibytes(value: f64) -> Self {
        Self((value * 1_073_741_824.0).round().max(0.0) as u64)
    }

    pub fn from_tebibytes(value: f64) -> Self {
        Self((value * 1_099_511_627_776.0).round().max(0.0) as u64)
    }

    pub fn from_pebibytes(value: f64) -> Self {
        Self((value * 1_125_899_906_842_624.0).round().max(0.0) as u64)
    }

    pub fn from_exbibytes(value: f64) -> Self {
        Self((value * 1_152_921_504_606_846_976.0).round().max(0.0) as u64)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn bytes(&self) -> u64 {
        self.0
    }

    pub fn kilobytes(&self) -> f64 {
        self.0 as f64 / 1_000.0
    }

    pub fn megabytes(&self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }

    pub fn gigabytes(&self) -> f64 {
        self.0 as f64 / 1_000_000_000.0
    }

    pub fn terabytes(&self) -> f64 {
        self.0 as f64 / 1_000_000_000_000.0
    }

    pub fn petabytes(&self) -> f64 {
        self.0 as f64 / 1_000_000_000_000_000.0
    }

    pub fn exabytes(&self) -> f64 {
        self.0 as f64 / 1_000_000_000_000_000_000.0
    }

    pub fn kibibytes(&self) -> f64 {
        self.0 as f64 / 1024.0
    }

    pub fn mebibytes(&self) -> f64 {
        self.0 as f64 / 1_048_576.0
    }

    pub fn gibibytes(&self) -> f64 {
        self.0 as f64 / 1_073_741_824.0
    }

    pub fn tebibytes(&self) -> f64 {
        self.0 as f64 / 1_099_511_627_776.0
    }

    pub fn pebibytes(&self) -> f64 {
        self.0 as f64 / 1_125_899_906_842_624.0
    }

    pub fn exbibytes(&self) -> f64 {
        self.0 as f64 / 1_152_921_504_606_846_976.0
    }

    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let input = input.trim();
        if input.is_empty() {
            return Err(ParseError::Empty);
        }

        let mut numeric_end = 0;
        for (index, ch) in input.char_indices() {
            if ch.is_ascii_digit() || ch == '.' || ch == '_' || ch == ',' {
                numeric_end = index + ch.len_utf8();
            } else {
                break;
            }
        }

        if numeric_end == 0 {
            return Err(ParseError::InvalidNumber);
        }

        let (number_part, unit_part) = input.split_at(numeric_end);
        let cleaned: String = number_part
            .chars()
            .filter(|ch| *ch != '_' && *ch != ',')
            .collect();
        let value: f64 = cleaned.parse().map_err(|_| ParseError::InvalidNumber)?;
        if !value.is_finite() || value < 0.0 {
            return Err(ParseError::InvalidNumber);
        }

        let unit = unit_part.trim().to_ascii_lowercase();
        let multiplier = unit_multiplier(&unit)?;
        let total = value * multiplier;
        if !total.is_finite() || total < 0.0 || total > u64::MAX as f64 {
            return Err(ParseError::Overflow);
        }

        Ok(ByteSize(total.round() as u64))
    }

    pub fn format(&self, system: UnitSystem, precision: usize) -> String {
        let (scaled, unit) = self.scaled(system);

        if unit == "B" {
            format!("{} B", self.0)
        } else {
            format!("{:.*} {}", precision, scaled, unit)
        }
    }

    pub fn scaled(&self, system: UnitSystem) -> (f64, &'static str) {
        let (base, units): (f64, &[&str]) = match system {
            UnitSystem::Binary => (1024.0, &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"]),
            UnitSystem::Decimal => (1000.0, &["B", "KB", "MB", "GB", "TB", "PB", "EB"]),
        };

        let mut exponent = 0usize;
        let mut scaled = self.0 as f64;
        while exponent < units.len() - 1 && scaled >= base {
            scaled /= base;
            exponent += 1;
        }

        (scaled, units[exponent])
    }
}

impl UnitSystem {
    pub fn as_str(self) -> &'static str {
        match self {
            UnitSystem::Binary => "binary",
            UnitSystem::Decimal => "decimal",
        }
    }

    pub fn parse(input: &str) -> Result<Self, ParseError> {
        match input.trim().to_ascii_lowercase().as_str() {
            "binary" | "bin" => Ok(UnitSystem::Binary),
            "decimal" | "dec" => Ok(UnitSystem::Decimal),
            _ => Err(ParseError::UnknownUnit),
        }
    }
}

impl FromStr for UnitSystem {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        UnitSystem::parse(input)
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format(UnitSystem::Binary, 2))
    }
}

impl fmt::Display for UnitSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Add for ByteSize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        ByteSize(self.0.saturating_add(rhs.0))
    }
}

impl AddAssign for ByteSize {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.saturating_add(rhs.0);
    }
}

impl Sub for ByteSize {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        ByteSize(self.0.saturating_sub(rhs.0))
    }
}

impl SubAssign for ByteSize {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.saturating_sub(rhs.0);
    }
}

impl Mul<u64> for ByteSize {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self {
        ByteSize(self.0.saturating_mul(rhs))
    }
}

impl Mul<ByteSize> for u64 {
    type Output = ByteSize;
    fn mul(self, rhs: ByteSize) -> ByteSize {
        rhs * self
    }
}

impl MulAssign<u64> for ByteSize {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 = self.0.saturating_mul(rhs);
    }
}

impl Div<u64> for ByteSize {
    type Output = Self;
    fn div(self, rhs: u64) -> Self {
        ByteSize(self.0 / rhs.max(1))
    }
}

impl DivAssign<u64> for ByteSize {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs.max(1);
    }
}

fn unit_multiplier(unit: &str) -> Result<f64, ParseError> {
    let value = match unit {
        "" | "b" | "byte" | "bytes" => 1.0,
        "k" | "ki" | "kib" | "kibibyte" | "kibibytes" => 1024.0,
        "kb" | "kilobyte" | "kilobytes" => 1000.0,
        "m" | "mi" | "mib" | "mebibyte" | "mebibytes" => 1024.0 * 1024.0,
        "mb" | "megabyte" | "megabytes" => 1000.0 * 1000.0,
        "g" | "gi" | "gib" | "gibibyte" | "gibibytes" => {
            1024.0 * 1024.0 * 1024.0
        }
        "gb" | "gigabyte" | "gigabytes" => 1000.0 * 1000.0 * 1000.0,
        "t" | "ti" | "tib" | "tebibyte" | "tebibytes" => {
            1024.0 * 1024.0 * 1024.0 * 1024.0
        }
        "tb" | "terabyte" | "terabytes" => {
            1000.0 * 1000.0 * 1000.0 * 1000.0
        }
        "p" | "pi" | "pib" | "pebibyte" | "pebibytes" => {
            1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0
        }
        "pb" | "petabyte" | "petabytes" => {
            1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0
        }
        "e" | "ei" | "eib" | "exbibyte" | "exbibytes" => {
            1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0
        }
        "eb" | "exabyte" | "exabytes" => {
            1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0
        }
        _ => return Err(ParseError::UnknownUnit),
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_bytes() {
        assert_eq!(ByteSize::parse("1024").unwrap(), ByteSize(1024));
    }

    #[test]
    fn parses_binary_units() {
        assert_eq!(ByteSize::parse("1.5 MiB").unwrap(), ByteSize(1_572_864));
        assert_eq!(ByteSize::parse("2KiB").unwrap(), ByteSize(2048));
    }

    #[test]
    fn parses_decimal_units() {
        assert_eq!(ByteSize::parse("1.5 MB").unwrap(), ByteSize(1_500_000));
        assert_eq!(ByteSize::parse("2KB").unwrap(), ByteSize(2000));
    }

    #[test]
    fn rejects_bad_input() {
        assert_eq!(ByteSize::parse("").unwrap_err(), ParseError::Empty);
        assert_eq!(ByteSize::parse("12 xyz").unwrap_err(), ParseError::UnknownUnit);
        assert_eq!(ByteSize::parse("nope").unwrap_err(), ParseError::InvalidNumber);
    }

    #[test]
    fn formats_values() {
        assert_eq!(ByteSize(1024).format(UnitSystem::Binary, 2), "1.00 KiB");
        assert_eq!(ByteSize(1500).format(UnitSystem::Decimal, 1), "1.5 KB");
        assert_eq!(ByteSize(512).format(UnitSystem::Binary, 2), "512 B");
    }

    #[test]
    fn display_uses_binary() {
        assert_eq!(ByteSize(1024).to_string(), "1.00 KiB");
    }

    #[test]
    fn from_str_matches_parse() {
        assert_eq!("2.5 GiB".parse::<ByteSize>().unwrap(), ByteSize::parse("2.5 GiB").unwrap());
        assert_eq!("".parse::<ByteSize>().unwrap_err(), ParseError::Empty);
    }

    #[test]
    fn arithmetic_operations() {
        assert_eq!(ByteSize(100) + ByteSize(200), ByteSize(300));
        assert_eq!(ByteSize(300) - ByteSize(100), ByteSize(200));
        assert_eq!(ByteSize(100) * 3, ByteSize(300));
        assert_eq!(ByteSize(300) / 3, ByteSize(100));
    }

    #[test]
    fn unit_system_from_str() {
        assert_eq!("binary".parse::<UnitSystem>().unwrap(), UnitSystem::Binary);
        assert_eq!("dec".parse::<UnitSystem>().unwrap(), UnitSystem::Decimal);
        assert!(UnitSystem::parse("nope").is_err());
    }

    #[test]
    fn compound_assignment_operations() {
        let mut size = ByteSize(10);
        size += ByteSize(5);
        assert_eq!(size, ByteSize(15));
        size -= ByteSize(3);
        assert_eq!(size, ByteSize(12));
        size *= 2;
        assert_eq!(size, ByteSize(24));
        size /= 4;
        assert_eq!(size, ByteSize(6));
    }

    #[test]
    fn u64_multiplies_byte_size() {
        assert_eq!(2 * ByteSize(3), ByteSize(6));
    }

    #[test]
    fn accesses_byte_count() {
        assert_eq!(ByteSize(42).as_u64(), 42);
        assert_eq!(ByteSize(42).bytes(), 42);
        assert_eq!(ByteSize::ZERO, ByteSize(0));
    }

    #[test]
    fn parse_error_is_std_error() {
        let error: Box<dyn std::error::Error> = Box::new(ParseError::Empty);
        assert_eq!(error.to_string(), "input is empty");
    }

    #[test]
    fn unit_conversion_helpers() {
        assert!((ByteSize(1_000_000).kilobytes() - 1000.0).abs() < f64::EPSILON);
        assert!((ByteSize(1024).kibibytes() - 1.0).abs() < f64::EPSILON);
        assert!((ByteSize(1_073_741_824).gibibytes() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn additional_unit_conversion_helpers() {
        assert!((ByteSize(1_000_000_000_000).terabytes() - 1.0).abs() < 1e-9);
        assert!((ByteSize(1_000_000_000_000_000).petabytes() - 1.0).abs() < 1e-9);
        assert!((ByteSize(1_000_000_000_000_000_000).exabytes() - 1.0).abs() < 1e-9);
        assert!((ByteSize(1_099_511_627_776).tebibytes() - 1.0).abs() < 1e-9);
        assert!((ByteSize(1_125_899_906_842_624).pebibytes() - 1.0).abs() < 1e-9);
        assert!((ByteSize(1_152_921_504_606_846_976).exbibytes() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn from_u64_and_to_u64() {
        assert_eq!(ByteSize::from(42u64), ByteSize(42));
        assert_eq!(u64::from(ByteSize(42)), 42);
    }

    #[test]
    fn from_unit_constructors() {
        assert_eq!(ByteSize::from_kilobytes(1.0), ByteSize(1000));
        assert_eq!(ByteSize::from_kibibytes(1.0), ByteSize(1024));
        assert_eq!(ByteSize::from_exabytes(1.0), ByteSize(1_000_000_000_000_000_000));
        assert_eq!(ByteSize::from_exbibytes(1.0), ByteSize(1_152_921_504_606_846_976));
    }

    #[test]
    fn parses_full_unit_names() {
        assert_eq!(ByteSize::parse("1 kibibyte").unwrap(), ByteSize(1024));
        assert_eq!(ByteSize::parse("1.5 megabytes").unwrap(), ByteSize(1_500_000));
        assert_eq!(ByteSize::parse("2 terabytes").unwrap(), ByteSize(2_000_000_000_000));
        assert_eq!(ByteSize::parse("3 pebibytes").unwrap(), ByteSize(3_377_699_720_527_872));
    }

    #[test]
    fn scaled_values() {
        assert_eq!(ByteSize(1024).scaled(UnitSystem::Binary), (1.0, "KiB"));
        assert_eq!(ByteSize(512).scaled(UnitSystem::Binary), (512.0, "B"));
        assert_eq!(ByteSize(1500).scaled(UnitSystem::Decimal), (1.5, "KB"));
    }
}
