use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct PgVersion {
    pub major: i32,
    pub minor: Option<i32>,
    pub patch: Option<i32>,
}

impl PgVersion {
    #[must_use]
    pub fn new(major: i32, minor: Option<i32>, patch: Option<i32>) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidNumber {
    pub version: String,
    pub e: ParseIntError,
}

#[derive(Debug, PartialEq)]
pub struct EmptyVersion {
    pub version: String,
}

#[derive(Debug, PartialEq)]
pub enum ParseVersionError {
    EmptyVersion(EmptyVersion),
    InvalidNumber(InvalidNumber),
}

impl std::fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::EmptyVersion(ref err) => {
                write!(f, "Empty version number provided: {:?}", err.version)
            }
            Self::InvalidNumber(ref err) => {
                write!(
                    f,
                    "Invalid number in version: {:?}. Parse error: {}",
                    err.version, err.e
                )
            }
        }
    }
}

fn parse_int(s: &str) -> Result<i32, ParseVersionError> {
    Ok(s.parse().map_err(|e| {
        ParseVersionError::InvalidNumber(InvalidNumber {
            version: s.to_string(),
            e,
        })
    }))?
}

impl FromStr for PgVersion {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_pieces: Vec<&str> = s.split('.').collect();

        if version_pieces.is_empty() {
            return Err(ParseVersionError::EmptyVersion(EmptyVersion {
                version: s.to_string(),
            }));
        }
        let major = parse_int(version_pieces[0])?;

        let minor: Option<i32> = if version_pieces.len() > 1 {
            Some(parse_int(version_pieces[1])?)
        } else {
            None
        };
        let patch: Option<i32> = if version_pieces.len() > 2 {
            Some(parse_int(version_pieces[2])?)
        } else {
            None
        };

        Ok(PgVersion {
            major,
            minor,
            patch,
        })
    }
}

#[cfg(test)]
mod test_pg_version {
    #![allow(clippy::neg_cmp_op_on_partial_ord)]
    use insta::assert_debug_snapshot;

    use super::*;
    #[test]
    fn test_eq() {
        assert_eq!(
            PgVersion::new(10, None, None),
            PgVersion::new(10, None, None)
        );
    }
    #[test]
    fn test_gt() {
        assert!(PgVersion::new(10, Some(1), None) > PgVersion::new(10, None, None));
        assert!(PgVersion::new(10, None, Some(1)) > PgVersion::new(10, None, None));
        assert!(PgVersion::new(10, None, Some(1)) > PgVersion::new(9, None, None));

        assert!(!(PgVersion::new(10, None, None) > PgVersion::new(10, None, None)));
    }
    #[test]
    fn test_parse() {
        assert_eq!(
            PgVersion::from_str("10.1"),
            Ok(PgVersion::new(10, Some(1), None))
        );
        assert_eq!(
            PgVersion::from_str("10"),
            Ok(PgVersion::new(10, None, None))
        );
        assert_eq!(
            PgVersion::from_str("10.2.1"),
            Ok(PgVersion::new(10, Some(2), Some(1)))
        );
        assert_debug_snapshot!(PgVersion::from_str("test"));
    }
}
