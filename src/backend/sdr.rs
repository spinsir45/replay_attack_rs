use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SDRSettings {
    pub center_frequency: f32,
    pub sample_rate: f32,
    pub tx_gain: u8,
    pub rx_gain: u8,
}

impl Default for SDRSettings {
    fn default() -> Self {
        SDRSettings {
            center_frequency: 915.0,
            sample_rate: 15.0,
            tx_gain: 1,
            rx_gain: 1,
        }
    }
}


