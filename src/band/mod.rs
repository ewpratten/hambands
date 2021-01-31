pub mod types;
use self::types::Band;

/// List of all valid amateur radio bands
pub const ALL_BANDS: [Band; 21] = [
    Band {
        name: "2190m",
        low_frequency: 135700,
        high_frequency: 137800,
    },
    Band {
        name: "630m",
        low_frequency: 472000,
        high_frequency: 479000,
    },
    Band {
        name: "160m",
        low_frequency: 1810000,
        high_frequency: 2000000,
    },
    Band {
        name: "80m",
        low_frequency: 3500000,
        high_frequency: 3800000,
    },
    Band {
        name: "60m",
        low_frequency: 5351500,
        high_frequency: 5366500,
    },
    Band {
        name: "40m",
        low_frequency: 7000000,
        high_frequency: 7200000,
    },
    Band {
        name: "30m",
        low_frequency: 10100000,
        high_frequency: 10150000,
    },
    Band {
        name: "20m",
        low_frequency: 14000000,
        high_frequency: 14350000,
    },
    Band {
        name: "17m",
        low_frequency: 18068000,
        high_frequency: 18168000,
    },
    Band {
        name: "15m",
        low_frequency: 21000000,
        high_frequency: 21450000,
    },
    Band {
        name: "12m",
        low_frequency: 24890000,
        high_frequency: 24990000,
    },
    Band {
        name: "10m",
        low_frequency: 28000000,
        high_frequency: 29700000,
    },
    Band {
        name: "6m",
        low_frequency: 50030000,
        high_frequency: 51000000,
    },
    Band {
        name: "4m",
        low_frequency: 70150000,
        high_frequency: 51000000,
    },
    Band {
        name: "2m",
        low_frequency: 144000000,
        high_frequency: 146000000,
    },
    Band {
        name: "70cm",
        low_frequency: 430000000,
        high_frequency: 440000000,
    },
    Band {
        name: "23cm",
        low_frequency: 1240000000,
        high_frequency: 1300000000,
    },
    Band {
        name: "13cm",
        low_frequency: 2320000000,
        high_frequency: 2450000000,
    },
    Band {
        name: "9cm",
        low_frequency: 3400000000,
        high_frequency: 3475000000,
    },
    Band {
        name: "6cm",
        low_frequency: 5650000000,
        high_frequency: 5850000000,
    },
    Band {
        name: "3cm",
        low_frequency: 10000000000,
        high_frequency: 10500000000,
    },
];
