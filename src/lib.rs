use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl ByteSize {
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

        if exponent == 0 {
            format!("{} B", self.0)
        } else {
            format!("{:.*} {}", precision, scaled, units[exponent])
        }
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format(UnitSystem::Binary, 2))
    }
}

fn unit_multiplier(unit: &str) -> Result<f64, ParseError> {
    let value = match unit {
        "" | "b" | "byte" | "bytes" => 1.0,
        "k" | "ki" | "kib" => 1024.0,
        "kb" => 1000.0,
        "m" | "mi" | "mib" => 1024.0 * 1024.0,
        "mb" => 1000.0 * 1000.0,
        "g" | "gi" | "gib" => 1024.0 * 1024.0 * 1024.0,
        "gb" => 1000.0 * 1000.0 * 1000.0,
        "t" | "ti" | "tib" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        "tb" => 1000.0 * 1000.0 * 1000.0 * 1000.0,
        "p" | "pi" | "pib" => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        "pb" => 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0,
        "e" | "ei" | "eib" => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        "eb" => 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0,
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
}
