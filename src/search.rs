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


