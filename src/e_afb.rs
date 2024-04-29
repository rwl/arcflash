use std::fmt::{Display, Formatter};
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::f64::{Length, RadiantExposure, Time};
use uom::si::length::millimeter;
use uom::si::radiant_exposure::joule_per_square_centimeter;
use uom::si::time::millisecond;

use crate::common::NominalVoltage;
use crate::cubicle::Cubicle;
use crate::equations::{intermediate_afb_from_e, intermediate_e, interpolate_e, interpolate_l};
use crate::i_arc::IArc;

pub enum EAfb {
    HighVoltage(EAfbHV),
    LowVoltage(EAfbLV),
}

impl EAfb {
    pub fn t_arc(&self) -> Time {
        match self {
            EAfb::HighVoltage(e_afb) => e_afb.t_arc,
            EAfb::LowVoltage(e_afb) => e_afb.t_arc,
        }
    }

    pub fn afb(&self) -> Length {
        match self {
            EAfb::HighVoltage(e_afb) => e_afb.afb,
            EAfb::LowVoltage(e_afb) => e_afb.afb,
        }
    }

    pub fn e(&self) -> RadiantExposure {
        match self {
            EAfb::HighVoltage(e_afb) => e_afb.e,
            EAfb::LowVoltage(e_afb) => e_afb.e,
        }
    }
}

pub struct EAfbHV {
    pub t_arc: Time,
    pub afb_14300: Length,
    pub afb_2700: Length,
    pub afb_600: Length,
    pub afb: Length,
    pub e_14300: RadiantExposure,
    pub e_2700: RadiantExposure,
    pub e_600: RadiantExposure,
    pub e: RadiantExposure,
}

pub struct EAfbLV {
    pub t_arc: Time,
    pub afb: Length,
    pub e: RadiantExposure,
}

pub fn e_afb(c: &Cubicle, i_arc: IArc, t_arc: Time) -> EAfb {
    // if c.hv {
    match i_arc {
        IArc::HighVoltage(i_arc) => {
            // Max
            let e_600 = intermediate_e(
                c,
                NominalVoltage::V0_6,
                i_arc.i_arc_600,
                i_arc.i_bf,
                t_arc,
                None,
            );
            let e_2700 = intermediate_e(
                c,
                NominalVoltage::V2_7,
                i_arc.i_arc_2700,
                i_arc.i_bf,
                t_arc,
                None,
            );
            let e_14300 = intermediate_e(
                c,
                NominalVoltage::V14_3,
                i_arc.i_arc_14300,
                i_arc.i_bf,
                t_arc,
                None,
            );
            let afb_600 = intermediate_afb_from_e(c, NominalVoltage::V0_6, e_600);
            let afb_2700 = intermediate_afb_from_e(c, NominalVoltage::V2_7, e_2700);
            let afb_14300 = intermediate_afb_from_e(c, NominalVoltage::V14_3, e_14300);

            EAfb::HighVoltage(EAfbHV {
                t_arc,
                afb_14300,
                afb_2700,
                afb_600,
                afb: interpolate_l(c, afb_600, afb_2700, afb_14300),
                e_14300,
                e_2700,
                e_600,
                e: interpolate_e(c, e_600, e_2700, e_14300),
            })
        }
        IArc::LowVoltage(i_arc) => {
            // Note I_arc_600_max, **not** I_arc_600_min, even in a "min" calculation.
            let e = intermediate_e(
                c,
                NominalVoltage::V0_6,
                i_arc.i_arc,
                i_arc.i_bf,
                t_arc,
                Some(i_arc.i_arc_600),
            );
            EAfb::LowVoltage(EAfbLV {
                t_arc,
                e,
                afb: intermediate_afb_from_e(c, NominalVoltage::V0_6, e),
            })
        }
    }
}

impl Display for EAfb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "T_arc = {}, E = {}, AFB = {}",
            self.t_arc().into_format_args(millisecond, Abbreviation),
            self.e()
                .into_format_args(joule_per_square_centimeter, Abbreviation),
            // self.e().get::<calorie_per_sq_cm>(),
            self.afb().into_format_args(millimeter, Abbreviation)
        )
    }
}
