use serde::{Serialize, Deserialize};
use crate::backend::sdr::SDRSettings;
use ndarray::Array1;

#[derive(Serialize, Deserialize)]
pub struct Signal {
    sdr_settings: SDRSettings,
    signal_data: Array1<f64>,
}
