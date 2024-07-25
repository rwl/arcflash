use crate::common::{ElectrodeConfiguration, NominalVoltage};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TABLE_1: HashMap<(ElectrodeConfiguration, NominalVoltage), Table1Row> = {
        HashMap::from([
            (
                (ElectrodeConfiguration::VCB, NominalVoltage::V600),
                Table1Row::new(
                    -0.04287, 1.035, -0.083, 0.0, 0.0, -4.783E-09, 1.962E-06, -0.000229, 0.003141,
                    1.092,
                ),
            ),
            (
                (ElectrodeConfiguration::VCB, NominalVoltage::V2700),
                Table1Row::new(
                    0.0065, 1.001, -0.024, -1.557E-12, 4.556E-10, -4.186E-08, 8.346E-07, 5.482E-05,
                    -0.003191, 0.9729,
                ),
            ),
            (
                (ElectrodeConfiguration::VCB, NominalVoltage::V14300),
                Table1Row::new(
                    0.005795, 1.015, -0.011, -1.557E-12, 4.556E-10, -4.186E-08, 8.346E-07,
                    5.482E-05, -0.003191, 0.9729,
                ),
            ),
            (
                (ElectrodeConfiguration::VCBB, NominalVoltage::V600),
                Table1Row::new(
                    -0.017432, 0.98, -0.05, 0.0, 0.0, -5.767E-09, 2.524E-06, -0.00034, 0.01187,
                    1.013,
                ),
            ),
            (
                (ElectrodeConfiguration::VCBB, NominalVoltage::V2700),
                Table1Row::new(
                    0.002823, 0.995, -0.0125, 0.0, -9.204E-11, 2.901E-08, -3.262E-06, 0.0001569,
                    -0.004003, 0.9825,
                ),
            ),
            (
                (ElectrodeConfiguration::VCBB, NominalVoltage::V14300),
                Table1Row::new(
                    0.014827, 1.01, -0.01, 0.0, -9.204E-11, 2.901E-08, -3.262E-06, 0.0001569,
                    -0.004003, 0.9825,
                ),
            ),
            (
                (ElectrodeConfiguration::HCB, NominalVoltage::V600),
                Table1Row::new(
                    0.054922, 0.988, -0.11, 0.0, 0.0, -5.382E-09, 2.316E-06, -0.000302, 0.0091,
                    0.9725,
                ),
            ),
            (
                (ElectrodeConfiguration::HCB, NominalVoltage::V2700),
                Table1Row::new(
                    0.001011, 1.003, -0.0249, 0.0, 0.0, 4.859E-10, -1.814E-07, -9.128E-06, -0.0007,
                    0.9881,
                ),
            ),
            (
                (ElectrodeConfiguration::HCB, NominalVoltage::V14300),
                Table1Row::new(
                    0.008693, 0.999, -0.02, 0.0, -5.043E-11, 2.233E-08, -3.046E-06, 0.000116,
                    -0.001145, 0.9839,
                ),
            ),
            (
                (ElectrodeConfiguration::VOA, NominalVoltage::V600),
                Table1Row::new(
                    0.043785, 1.04, -0.18, 0.0, 0.0, -4.783E-09, 1.962E-06, -0.000229, 0.003141,
                    1.092,
                ),
            ),
            (
                (ElectrodeConfiguration::VOA, NominalVoltage::V2700),
                Table1Row::new(
                    -0.02395, 1.006, -0.0188, -1.557E-12, 4.556E-10, -4.186E-08, 8.346E-07,
                    5.482E-05, -0.003191, 0.9729,
                ),
            ),
            (
                (ElectrodeConfiguration::VOA, NominalVoltage::V14300),
                Table1Row::new(
                    0.005371, 1.0102, -0.029, -1.557E-12, 4.556E-10, -4.186E-08, 8.346E-07,
                    5.482E-05, -0.003191, 0.9729,
                ),
            ),
            (
                (ElectrodeConfiguration::HOA, NominalVoltage::V600),
                Table1Row::new(
                    0.111147, 1.008, -0.24, 0.0, 0.0, -3.895E-09, 1.641E-06, -0.000197, 0.002615,
                    1.1,
                ),
            ),
            (
                (ElectrodeConfiguration::HOA, NominalVoltage::V2700),
                Table1Row::new(
                    0.000435, 1.006, -0.038, 0.0, 0.0, 7.859E-10, -1.914E-07, -9.128E-06, -0.0007,
                    0.9981,
                ),
            ),
            (
                (ElectrodeConfiguration::HOA, NominalVoltage::V14300),
                Table1Row::new(
                    0.000904, 0.999, -0.02, 0.0, 0.0, 7.859E-10, -1.914E-07, -9.128E-06, -0.0007,
                    0.9981,
                ),
            ),
        ])
    };
}

pub struct Table1Row {
    // ec: String,
    // v_oc: f64,
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
}

impl Table1Row {
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
        }
    }
}
