use crate::common::ElectrodeConfiguration;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TABLE_3: HashMap<ElectrodeConfiguration, Table3_4_5Row> = {
        HashMap::from([
            (
                ElectrodeConfiguration::VCB,
                Table3_4_5Row::new(
                    0.753364,
                    0.566,
                    1.752636,
                    0.0,
                    0.0,
                    -4.783E-09,
                    0.000001962,
                    -0.000229,
                    0.003141,
                    1.092,
                    0.0,
                    -1.598,
                    0.957,
                ),
            ),
            (
                ElectrodeConfiguration::VCBB,
                Table3_4_5Row::new(
                    3.068459,
                    0.26,
                    -0.098107,
                    0.0,
                    0.0,
                    -5.767E-09,
                    0.000002524,
                    -0.00034,
                    0.01187,
                    1.013,
                    -0.06,
                    -1.809,
                    1.19,
                ),
            ),
            (
                ElectrodeConfiguration::HCB,
                Table3_4_5Row::new(
                    4.073745,
                    0.344,
                    -0.370259,
                    0.0,
                    0.0,
                    -5.382E-09,
                    0.000002316,
                    -0.000302,
                    0.0091,
                    0.9725,
                    0.0,
                    -2.03,
                    1.036,
                ),
            ),
            (
                ElectrodeConfiguration::VOA,
                Table3_4_5Row::new(
                    0.679294,
                    0.746,
                    1.222636,
                    0.0,
                    0.0,
                    -4.783E-09,
                    0.000001962,
                    -0.000229,
                    0.003141,
                    1.092,
                    0.0,
                    -1.598,
                    0.997,
                ),
            ),
            (
                ElectrodeConfiguration::HOA,
                Table3_4_5Row::new(
                    3.470417,
                    0.465,
                    -0.261863,
                    0.0,
                    0.0,
                    -3.895E-09,
                    0.000001641,
                    -0.000197,
                    0.002615,
                    1.1,
                    0.0,
                    -1.99,
                    1.04,
                ),
            ),
        ])
    };
    pub static ref TABLE_4: HashMap<ElectrodeConfiguration, Table3_4_5Row> = {
        HashMap::from([
            (
                ElectrodeConfiguration::VCB,
                Table3_4_5Row::new(
                    2.40021, 0.165, 0.354202, -1.557E-12, 4.556E-10, -4.186E-08, 8.346E-07,
                    5.482E-05, -0.003191, 0.9729, 0.0, -1.569, 0.9778,
                ),
            ),
            (
                ElectrodeConfiguration::VCBB,
                Table3_4_5Row::new(
                    3.870592, 0.185, -0.736618, 0.0, -9.204E-11, 2.901E-08, -3.262E-06, 0.0001569,
                    -0.004003, 0.9825, 0.0, -1.742, 1.09,
                ),
            ),
            (
                ElectrodeConfiguration::HCB,
                Table3_4_5Row::new(
                    3.486391, 0.177, -0.193101, 0.0, 0.0, 4.859E-10, -1.814E-07, -9.128E-06,
                    -0.0007, 0.9881, 0.027, -1.723, 1.055,
                ),
            ),
            (
                ElectrodeConfiguration::VOA,
                Table3_4_5Row::new(
                    3.880724, 0.105, -1.906033, -1.557E-12, 4.556E-10, -4.186E-08, 8.346E-07,
                    5.482E-05, -0.003191, 0.9729, 0.0, -1.515, 1.115,
                ),
            ),
            (
                ElectrodeConfiguration::HOA,
                Table3_4_5Row::new(
                    3.616266, 0.149, -0.761561, 0.0, 0.0, 7.859E-10, -1.914E-07, -9.128E-06,
                    -0.0007, 0.9981, 0.0, -1.639, 1.078,
                ),
            ),
        ])
    };
    pub static ref TABLE_5: HashMap<ElectrodeConfiguration, Table3_4_5Row> = {
        HashMap::from([
            (
                ElectrodeConfiguration::VCB,
                Table3_4_5Row::new(
                    3.825917, 0.11, -0.999749, -1.557E-12, 4.556E-10, -4.186E-08, 8.346E-07,
                    5.482E-05, -0.003191, 0.9729, 0.0, -1.568, 0.99,
                ),
            ),
            (
                ElectrodeConfiguration::VCBB,
                Table3_4_5Row::new(
                    3.644309, 0.215, -0.585522, 0.0, -9.204E-11, 2.901E-08, -3.262E-06, 0.0001569,
                    -0.004003, 0.9825, 0.0, -1.677, 1.06,
                ),
            ),
            (
                ElectrodeConfiguration::HCB,
                Table3_4_5Row::new(
                    3.044516, 0.125, 0.245106, 0.0, -5.043E-11, 2.233E-08, -3.046E-06, 0.000116,
                    -0.001145, 0.9839, 0.0, -1.655, 1.084,
                ),
            ),
            (
                ElectrodeConfiguration::VOA,
                Table3_4_5Row::new(
                    3.405454, 0.12, -0.93245, -1.557E-12, 4.556E-10, -4.186E-08, 8.346E-07,
                    5.482E-05, -0.003191, 0.9729, 0.0, -1.534, 0.979,
                ),
            ),
            (
                ElectrodeConfiguration::HOA,
                Table3_4_5Row::new(
                    2.04049, 0.177, 1.005092, 0.0, 0.0, 7.859E-10, -1.914E-07, -9.128E-06, -0.0007,
                    0.9981, -0.05, -1.633, 1.151,
                ),
            ),
        ])
    };
}

pub struct Table3_4_5Row {
    // v600: String,
    pub k1: f64,
    pub k2: f64,
    pub k3: f64,
    pub k4: f64,
    pub k5: f64,
    pub k6: f64,
    pub k7: f64,
    pub k8: f64,
    pub k9: f64,
    pub k10: f64,
    pub k11: f64,
    pub k12: f64,
    pub k13: f64,
}

impl Table3_4_5Row {
    fn new(
        k1: f64,
        k2: f64,
        k3: f64,
        k4: f64,
        k5: f64,
        k6: f64,
        k7: f64,
        k8: f64,
        k9: f64,
        k10: f64,
        k11: f64,
        k12: f64,
        k13: f64,
    ) -> Self {
        Self {
            k1,
            k2,
            k3,
            k4,
            k5,
            k6,
            k7,
            k8,
            k9,
            k10,
            k11,
            k12,
            k13,
        }
    }
}
