pub mod types;
use self::types::Band;

/// List of all valid amateur radio bands
pub const ALL_BANDS: [Band; 21] = [
    Band {
        name: "2190m",
        low_frequency: 135_700,
        high_frequency: 137_800,
    },
    Band {
        name: "630m",
        low_frequency: 472_000,
        high_frequency: 479_000,
    },
    Band {
        name: "160m",
        low_frequency: 1_810_000,
        high_frequency: 2_000_000,
    },
    Band {
        name: "80m",
        low_frequency: 3_500_000,
        high_frequency: 3_800_000,
    },
    Band {
        name: "60m",
        low_frequency: 5_351_500,
        high_frequency: 5_366_500,
    },
    Band {
        name: "40m",
        low_frequency: 7_000_000,
        high_frequency: 7_200_000,
    },
    Band {
        name: "30m",
        low_frequency: 10_100_000,
        high_frequency: 10_150_000,
    },
    Band {
        name: "20m",
        low_frequency: 14_000_000,
        high_frequency: 14_350_000,
    },
    Band {
        name: "17m",
        low_frequency: 18_068_000,
        high_frequency: 18_168_000,
    },
    Band {
        name: "15m",
        low_frequency: 21_000_000,
        high_frequency: 21_450_000,
    },
    Band {
        name: "12m",
        low_frequency: 24_890_000,
        high_frequency: 24_990_000,
    },
    Band {
        name: "10m",
        low_frequency: 28_000_000,
        high_frequency: 29_700_000,
    },
    Band {
        name: "6m",
        low_frequency: 50_030_000,
        high_frequency: 51_000_000,
    },
    Band {
        name: "4m",
        low_frequency: 70_150_000,
        high_frequency: 70_200_000,
    },
    Band {
        name: "2m",
        low_frequency: 144_000_000,
        high_frequency: 147_990_000,
    },
    Band {
        name: "70cm",
        low_frequency: 430_000_000,
        high_frequency: 440_000_000,
    },
    Band {
        name: "23cm",
        low_frequency: 1_240_000_000,
        high_frequency: 1_300_000_000,
    },
    Band {
        name: "13cm",
        low_frequency: 2_320_000_000,
        high_frequency: 2_450_000_000,
    },
    Band {
        name: "9cm",
        low_frequency: 3_400_000_000,
        high_frequency: 3_475_000_000,
    },
    Band {
        name: "6cm",
        low_frequency: 5_650_000_000,
        high_frequency: 5_850_000_000,
    },
    Band {
        name: "3cm",
        low_frequency: 10_000_000_000,
        high_frequency: 10_500_000_000,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_data() {
        for band in ALL_BANDS.iter() {
            // Ensure the low frequency is below the high frequency
            if band.low_frequency >= band.high_frequency {
                panic!(format!(
                    "Low frequency {} >= high frequency {} on band: {}",
                    band.low_frequency, band.high_frequency, band.name
                ))
            }
        }
    }
}
