use anyhow::{format_err, Result};
use std::fmt::{Display, Formatter};
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::electric_current::kiloampere;
use uom::si::electric_potential::kilovolt;
use uom::si::f64::{ElectricCurrent, ElectricPotential};

use crate::common::NominalVoltage;
use crate::cubicle::Cubicle;
use crate::equations::{i_arc_final_lv, i_arc_intermediate, i_arc_min, interpolate};

#[derive(Clone)]
pub enum IArc {
    HighVoltage(IArcHV),
    LowVoltage(IArcLV),
}

impl IArc {
    pub fn hv(&self) -> &IArcHV {
        match self {
            IArc::HighVoltage(i_arc_hv) => i_arc_hv,
            IArc::LowVoltage(_) => {
                panic!("called `IArc::hv()` on a `LowVoltage` value")
            }
        }
    }

    pub fn lv(&self) -> &IArcLV {
        match self {
            IArc::LowVoltage(i_arc_lv) => i_arc_lv,
            IArc::HighVoltage(_) => {
                panic!("called `IArc::lv()` on a `HighVoltage` value")
            }
        }
    }

    pub fn i_bf(&self) -> ElectricCurrent {
        match self {
            IArc::HighVoltage(i_arc) => i_arc.i_bf,
            IArc::LowVoltage(i_arc) => i_arc.i_bf,
        }
    }

    pub fn reduced(&self) -> bool {
        match self {
            IArc::HighVoltage(i_arc) => i_arc.reduced,
            IArc::LowVoltage(i_arc) => i_arc.reduced,
        }
    }

    pub fn i_arc_600(&self) -> ElectricCurrent {
        match self {
            IArc::HighVoltage(i_arc) => i_arc.i_arc_600,
            IArc::LowVoltage(i_arc) => i_arc.i_arc_600,
        }
    }

    pub fn i_arc(&self) -> ElectricCurrent {
        match self {
            IArc::HighVoltage(i_arc) => i_arc.i_arc,
            IArc::LowVoltage(i_arc) => i_arc.i_arc,
        }
    }
}

#[derive(Clone)]
pub struct IArcHV {
    pub i_bf: ElectricCurrent,
    pub reduced: bool,
    pub i_arc_14300: ElectricCurrent,
    pub i_arc_2700: ElectricCurrent,
    pub i_arc_600: ElectricCurrent,
    pub i_arc: ElectricCurrent,
}

#[derive(Clone)]
pub struct IArcLV {
    pub i_bf: ElectricCurrent,
    pub reduced: bool,
    pub i_arc_600: ElectricCurrent,
    pub i_arc: ElectricCurrent,
}

/// If `reduced` is true `I_arc_min` is used, e.g. that the arcing current variation factor `var_cf` is used.
/// Otherwise, the full value of `I_arc` is used.
///
/// These are referred to as "full"/"reduced" to avoid confusion with "max"/"min" which mean e.g.
/// "maximum fault operating scenario" and "minimum fault operating scenario" in context.
pub fn i_arc(c: &Cubicle, i_bf: ElectricCurrent, reduced: bool) -> Result<IArc> {
    if (ElectricPotential::new::<kilovolt>(0.208) <= c.v_oc
        && c.v_oc <= ElectricPotential::new::<kilovolt>(0.600))
        && !(ElectricCurrent::new::<kiloampere>(0.500) <= i_bf
            && i_bf <= ElectricCurrent::new::<kiloampere>(106.000))
    {
        return Err(format_err!(
            "I_bf = {} is outside LV calculation range 500 A to 106 kA.",
            i_bf.into_format_args(kiloampere, Abbreviation)
        ));
    } else if (ElectricPotential::new::<kilovolt>(0.600) < c.v_oc
        && c.v_oc <= ElectricPotential::new::<kilovolt>(15.000))
        && !(ElectricCurrent::new::<kiloampere>(0.200) <= i_bf
            && i_bf <= ElectricCurrent::new::<kiloampere>(65.000))
    {
        return Err(format_err!(
            "I_bf = {} is outside HV calculation range 200 A to 65 kA.",
            i_bf.into_format_args(kiloampere, Abbreviation)
        ));
    }

    if c.hv {
        let i_arc_600_full = i_arc_intermediate(&c, NominalVoltage::V600, i_bf);
        let i_arc_2700_full = i_arc_intermediate(&c, NominalVoltage::V2700, i_bf);
        let i_arc_14300_full = i_arc_intermediate(&c, NominalVoltage::V14300, i_bf);
        if !reduced {
            let i_arc_600 = i_arc_600_full;
            let i_arc_2700 = i_arc_2700_full;
            let i_arc_14300 = i_arc_14300_full;

            Ok(IArc::HighVoltage(IArcHV {
                i_bf,
                reduced,
                i_arc_14300,
                i_arc_2700,
                i_arc_600,
                i_arc: interpolate!(&c, i_arc_600, i_arc_2700, i_arc_14300),
            }))
        } else {
            let i_arc_600 = i_arc_min(&c, i_arc_600_full);
            let i_arc_2700 = i_arc_min(&c, i_arc_2700_full);
            let i_arc_14300 = i_arc_min(&c, i_arc_14300_full);

            Ok(IArc::HighVoltage(IArcHV {
                i_bf,
                reduced,
                i_arc_14300,
                i_arc_2700,
                i_arc_600,
                i_arc: interpolate!(&c, i_arc_600, i_arc_2700, i_arc_14300),
            }))
        }
    } else {
        let i_arc_600 = i_arc_intermediate(&c, NominalVoltage::V600, i_bf);
        let i_arc_full = i_arc_final_lv(&c, i_arc_600, i_bf);

        let i_arc = if !reduced {
            i_arc_full
        } else {
            i_arc_min(&c, i_arc_full)
        };

        Ok(IArc::LowVoltage(IArcLV {
            i_bf,
            reduced,
            i_arc_600,
            i_arc,
        }))
    }
}

impl Display for IArc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "I_bf = {}, I_arc = {} ({})",
            self.i_bf().into_format_args(kiloampere, Abbreviation),
            self.i_arc().into_format_args(kiloampere, Abbreviation),
            if self.reduced() { "reduced" } else { "full" }
        )
    }
}
