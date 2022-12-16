pub mod types;
use self::types::Band;

/// List of all valid amateur radio bands
pub const ALL_BANDS: [Band; 30] = [
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
        name: "560m",
        low_frequency: 501_000,
        high_frequency: 504_000,
    },
    Band {
        name: "160m",
        low_frequency: 1_800_000,
        high_frequency: 2_000_000,
    },
    Band {
        name: "80m",
        low_frequency: 3_500_000,
        high_frequency: 4_000_000,
    },
    Band {
        name: "60m",
        low_frequency: 5_060_000,
        high_frequency: 5_450_000,
    },
    Band {
        name: "40m",
        low_frequency: 7_000_000,
        high_frequency: 7_300_000,
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
        low_frequency: 50_000_000,
        high_frequency: 54_000_000,
    },
    Band {
        name: "4m",
        low_frequency: 70_000_000,
        high_frequency: 71_000_000,
    },
    Band {
        name: "2m",
        low_frequency: 144_000_000,
        high_frequency: 148_000_000,
    },
    Band {
        name: "1.25m",
        low_frequency: 220_000_000,
        high_frequency: 225_000_000,
    },
    Band {
        name: "70cm",
        low_frequency: 420_000_000,
        high_frequency: 450_000_000,
    },
    Band {
        name: "33cm",
        low_frequency: 902_000_000,
        high_frequency: 928_000_000,
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
    Band {
        name: "1.25cm",
        low_frequency: 24_000_000_000,
        high_frequency: 24_250_000_000,
    },
    Band {
        name: "6mm",
        low_frequency: 47_000_000_000,
        high_frequency: 47_200_000_000,
    },
    Band {
        name: "4mm",
        low_frequency: 75_500_000_000,
        high_frequency: 81_000_000_000,
    },
    Band {
        name: "2.5mm",
        low_frequency: 119_980_000_000,
        high_frequency: 120_020_000_000,
    },
    Band {
        name: "2mm",
        low_frequency: 142_000_000_000,
        high_frequency: 149_000_000_000,
    },
    Band {
        name: "1mm",
        low_frequency: 241_000_000_000,
        high_frequency: 250_000_000_000,
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
