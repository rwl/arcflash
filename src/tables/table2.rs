use crate::common::ElectrodeConfiguration;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TABLE_2: HashMap<ElectrodeConfiguration, Table2Row> = {
        HashMap::from([
            (
                ElectrodeConfiguration::VCB,
                Table2Row::new(
                    0.0,
                    -0.0000014269,
                    0.000083137,
                    -0.0019382,
                    0.022366,
                    -0.12645,
                    0.30226,
                ),
            ),
            (
                ElectrodeConfiguration::VCBB,
                Table2Row::new(
                    1.138e-06,
                    -6.0287e-05,
                    0.0012758,
                    -0.013778,
                    0.080217,
                    -0.24066,
                    0.33524,
                ),
            ),
            (
                ElectrodeConfiguration::HCB,
                Table2Row::new(
                    0.0, -3.097e-06, 0.00016405, -0.0033609, 0.033308, -0.16182, 0.34627,
                ),
            ),
            (
                ElectrodeConfiguration::VOA,
                Table2Row::new(
                    9.5606E-07,
                    -5.1543E-05,
                    0.0011161,
                    -0.01242,
                    0.075125,
                    -0.23584,
                    0.33696,
                ),
            ),
            (
                ElectrodeConfiguration::HOA,
                Table2Row::new(
                    0.0,
                    -3.1555e-06,
                    0.0001682,
                    -0.0034607,
                    0.034124,
                    -0.1599,
                    0.34629,
                ),
            ),
        ])
    };
}

pub struct Table2Row {
    // ec: String,
    pub k1: f64,
    pub k2: f64,
    pub k3: f64,
    pub k4: f64,
    pub k5: f64,
    pub k6: f64,
    pub k7: f64,
}

impl Table2Row {
    fn new(k1: f64, k2: f64, k3: f64, k4: f64, k5: f64, k6: f64, k7: f64) -> Self {
        Self {
            k1,
            k2,
            k3,
            k4,
            k5,
            k6,
            k7,
        }
    }
}
