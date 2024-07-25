use uom::si::f64::{Length, RadiantExposure};

use crate::equations::{intermediate_afb_from_e, interpolate};
use crate::{Cubicle, EAfb, NominalVoltage};

/// Calculates the total energy and total arc flash boundary for a multistep calculation.
///
/// E.g. for example, if there are 2 calculation time steps -
/// Step 1: calculation for I_bf = 10kA for T = 0.100 sec -> E = 10 J/cm², AFB = 1,000 mm
/// Step 2: calculation for I_bf = 5kA for T = 0.200 sec -> E = 2 J/cm², AFB = 500 mm
///
/// The total energy is just the sum of energy at each step: total_E = 10 + 2 = 12 J/cm².
///
/// The arc flash boundary, however, is non-linear with energy and so needs special treatment.
///
/// For HV, intermediate AFB's are worked out from the intermediate E values at 600, 2700, and 14300 V.
///
/// So we need to calculate the sum of E_600, E_2700, and E_14300 across all steps, then calculate the intermediate
/// AFB values AFB_600, AFB_2700, and AFB_14300.
///
/// The final AFB is then interpolated from the intermediate AFB's.
///
/// For LV, there are no intermediate values / interpolation so we can just work out the AFB based on total_E.
pub fn multistep_e_and_afb(c: &Cubicle, calc_steps: &[EAfb]) -> (RadiantExposure, Length) {
    let total_e = calc_steps.iter().map(|e_afb| e_afb.e()).sum();

    let total_afb = if c.hv {
        let total_e_600 = calc_steps.iter().map(|e_afb| e_afb.hv().e_600).sum();
        let total_e_2700 = calc_steps.iter().map(|e_afb| e_afb.hv().e_2700).sum();
        let total_e_14300 = calc_steps.iter().map(|e_afb| e_afb.hv().e_14300).sum();

        let afb_600 = intermediate_afb_from_e(c, NominalVoltage::V600, total_e_600);
        let afb_2700 = intermediate_afb_from_e(c, NominalVoltage::V2700, total_e_2700);
        let afb_14300 = intermediate_afb_from_e(c, NominalVoltage::V14300, total_e_14300);

        interpolate!(c, afb_600, afb_2700, afb_14300)
    } else {
        intermediate_afb_from_e(c, NominalVoltage::V600, total_e)
    };

    (total_e, total_afb)
}
