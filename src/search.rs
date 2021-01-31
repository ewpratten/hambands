use crate::band::types::{Band, Hertz};
use crate::band::ALL_BANDS;
use crate::errors::SearchError;

/// Get the corresponding band for a frequency
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

/// Get a band by its name.
/// Example:
/// 
/// ```rust
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
