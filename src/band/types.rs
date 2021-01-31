/// Type aliases for frequencies
pub type Hertz = u64;
pub type KiloHertz = f32;
pub type MegaHertz = f32;
pub type GigaHertz = f32;

/// Data structure for storing information about a radio band
pub struct Band {
    /// Friendly name of the band
    pub name: &'static str,

    /// Lowest frequency
    pub low_frequency: Hertz,

    /// Highest frequency
    pub high_frequency: Hertz,
}
