use crate::band::types::{Band, Hertz};
use crate::band::ALL_BANDS;
use crate::errors::SearchError;

/// Get the corresponding band for a frequency
/// 
/// Example:
/// ```rust
/// # fn get_band_for_frequency(frequency: u64) {}
/// get_band_for_frequency(7_040_000);
/// get_band_for_frequency(146_350_000);
/// ```
pub fn get_band_for_frequency(frequency: Hertz) -> Result<&'static Band, SearchError> {
    // Search every band
    for band in ALL_BANDS.iter() {
        if band.low_frequency <= frequency && frequency <= band.high_frequency {
            return Ok(band);
        }
    }

    Err(SearchError::new(&format!(
        "Could not find any amateur radio bands containing frequency: {}",
        frequency
    )))
}

/// Get a band by its name
/// 
/// Example:
/// ```rust
/// # fn get_band_by_name(name: &str) {}
/// get_band_by_name("40m");
/// get_band_by_name("70cm");
/// ```
pub fn get_band_by_name(name: &str) -> Result<&'static Band, SearchError> {
    // Search every band
    for band in ALL_BANDS.iter() {
        if band.name == name {
            return Ok(band);
        }
    }

    Err(SearchError::new(&format!(
        "Not an amateur radio band name: {}",
        name
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_band_by_valid_freq() {
        let band = get_band_for_frequency(7_041_000);

        assert!(band.is_ok() && !band.is_err());
        assert_eq!(band.unwrap().name, "40m");
    }

    #[test]
    fn test_band_by_invalid_freq() {
        let band = get_band_for_frequency(8_000_000);

        assert!(band.is_err() && !band.is_ok());
    }
}
