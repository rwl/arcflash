use crate::common::{ElectrodeConfiguration, EnclosureType};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TABLE_7: HashMap<(EnclosureType, ElectrodeConfiguration), Table7Row> = {
        HashMap::from([
            (
                (EnclosureType::Typical, ElectrodeConfiguration::VCB),
                Table7Row::new(-0.000302, 0.03441, 0.4325),
            ),
            (
                (EnclosureType::Typical, ElectrodeConfiguration::VCBB),
                Table7Row::new(-0.0002976, 0.032, 0.479),
            ),
            (
                (EnclosureType::Typical, ElectrodeConfiguration::HCB),
                Table7Row::new(-0.0001923, 0.01935, 0.6899),
            ),
            (
                (EnclosureType::Shallow, ElectrodeConfiguration::VCB),
                Table7Row::new(0.002222, -0.02556, 0.6222),
            ),
            (
                (EnclosureType::Shallow, ElectrodeConfiguration::VCBB),
                Table7Row::new(-0.002778, 0.1194, -0.2778),
            ),
            (
                (EnclosureType::Shallow, ElectrodeConfiguration::HCB),
                Table7Row::new(-0.0005556, 0.03722, 0.4778),
            ),
        ])
    };
}

pub struct Table7Row {
    // box_type: String,
    // ec: String,
    pub b1: f64,
    pub b2: f64,
    pub b3: f64,
}

impl Table7Row {
    fn new(b1: f64, b2: f64, b3: f64) -> Self {
        Self { b1, b2, b3 }
    }
}
