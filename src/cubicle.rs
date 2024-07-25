// Copyright 2022, Li-aung Yip - https://www.penwatch.net
// Licensed under the MIT License. Refer LICENSE.txt.

use anyhow::{format_err, Result};
use std::fmt::{Display, Formatter};
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::electric_potential::kilovolt;
use uom::si::f64::*;
use uom::si::length::inch;
use uom::si::length::millimeter as mm;

use crate::common::{ElectrodeConfiguration, EnclosureType};
use crate::tables::{TABLE_2, TABLE_7};

/// Encapsulates physical parameters of equipment that do not change with current (kA) or time (ms).
pub struct Cubicle {
    /// nominal voltage
    pub v_oc: ElectricPotential,
    pub ec: ElectrodeConfiguration,
    /// busbar gap
    pub g: Length,
    /// working distance
    pub d: Length,

    pub dim: BoxDimensions,
    // escf: EnclosureCorrectionFactor,
    pub enclosure_type: EnclosureType,
    pub var_cf: f64,
    pub cf: f64,
    pub(crate) debug: Option<EnclosureDebug>,
    pub hv: bool,
}

pub(crate) struct EnclosureDebug {
    pub(crate) height: Length,
    pub(crate) width: Length,
    pub(crate) ees: f64,
}

impl Cubicle {
    pub fn new(
        v_oc: ElectricPotential,
        ec: ElectrodeConfiguration,
        g: Length,
        d: Length,
        height: Length,
        width: Length,
        depth: Length,
    ) -> Result<Self> {
        let var_cf = Self::calc_var_cf(v_oc, ec);

        let enclosure_type = if (v_oc < ElectricPotential::new::<kilovolt>(0.6))
            && (height < Length::new::<mm>(508.0))
            && (width < Length::new::<mm>(508.0))
            && (depth <= Length::new::<mm>(203.2))
        {
            EnclosureType::Shallow
        } else {
            EnclosureType::Typical
        };

        let (cf, debug) = Self::calc_cf(v_oc, ec, height, width, enclosure_type);

        Self::check_model_bounds(v_oc, g, d, width)?;

        if cf < 0.0 || cf > 3.0 {
            return Err(format_err!(
                "enclosure size correction factor ({}) must be between 0 and 3",
                cf
            ));
        }

        let hv = if ElectricPotential::new::<kilovolt>(0.600) < v_oc
            && v_oc <= ElectricPotential::new::<kilovolt>(15.000)
        {
            true
        } else if v_oc <= ElectricPotential::new::<kilovolt>(0.600) {
            false
        } else {
            unreachable!()
        };

        Ok(Self {
            v_oc,
            ec,
            g,
            d,
            dim: BoxDimensions {
                height,
                width,
                depth,
            },
            enclosure_type,
            var_cf,
            cf,
            debug,
            hv,
        })
    }

    // ref IEEE 1584-2018 s4.2 "Range of model"
    // Applying the IEEE 1584-2018 model outside these ranges _WILL_ give incorrect results.
    fn check_model_bounds(
        v_oc: ElectricPotential,
        g: Length,
        d: Length,
        width: Length,
    ) -> Result<()> {
        // assert 0.208 * kV <= self.V_oc <= 15 * kV

        if v_oc <= ElectricPotential::new::<kilovolt>(0.600) {
            // low voltage
            if g < Length::new::<mm>(6.35) {
                return Err(format_err!(
                    "busbar gap G must be >= 6.35 mm: {}",
                    g.into_format_args(mm, Abbreviation)
                ));
            }
            if g > Length::new::<mm>(76.2) {
                return Err(format_err!(
                    "busbar gap G must be <= 76.2 mm: {}",
                    g.into_format_args(mm, Abbreviation)
                ));
            }
        } else {
            // high voltage
            if g < Length::new::<mm>(19.05) {
                return Err(format_err!(
                    "busbar gap G must be >= 19.05 mm: {}",
                    g.into_format_args(mm, Abbreviation)
                ));
            }
            if g > Length::new::<mm>(254.0) {
                return Err(format_err!(
                    "busbar gap G must be <= 254 mm: {}",
                    g.into_format_args(mm, Abbreviation)
                ));
            }
        }
        if d < Length::new::<mm>(305.0) {
            return Err(format_err!(
                "working distance D must be >= 305 mm: {}",
                d.into_format_args(mm, Abbreviation)
            ));
        }

        if width < 4.0 * g {
            return Err(format_err!(
                "width of enclosure ({}) must be at least four times the busbar gap G (4 * {} = {})",
                width.into_format_args(mm, Abbreviation),
                g.into_format_args(mm, Abbreviation),
                (4.0 * g).into_format_args(mm, Abbreviation)
            ));
        }

        Ok(())
    }

    // Arcing current variation correction factor.
    // The equation under Equation 2. (Equation 2a?)
    fn calc_var_cf(v_oc: ElectricPotential, ec: ElectrodeConfiguration) -> f64 {
        // Specifically need V_oc to be in kV for this formula
        let v_oc = v_oc.get::<kilovolt>();

        let k = TABLE_2.get(&ec).unwrap();

        k.k1 * v_oc.powi(6)
            + k.k2 * v_oc.powi(5)
            + k.k3 * v_oc.powi(4)
            + k.k4 * v_oc.powi(3)
            + k.k5 * v_oc.powi(2)
            + k.k6 * v_oc.powi(1)
            + k.k7
    }

    // Enclosure size correction factor.
    fn calc_cf(
        v_oc: ElectricPotential,
        ec: ElectrodeConfiguration,
        h: Length,
        w: Length,
        enclosure_type: EnclosureType,
    ) -> (f64, Option<EnclosureDebug>) {
        if ec == ElectrodeConfiguration::HOA || ec == ElectrodeConfiguration::VOA {
            // Open air configurations HOA / VOA do not require a box size correction factor.
            return (1.0, None);
        }
        let (a, b) = match ec {
            ElectrodeConfiguration::VCB => (4.0, 20.0),
            ElectrodeConfiguration::VCBB => (10.0, 24.0),
            ElectrodeConfiguration::HCB => (10.0, 22.0),
            _ => {
                unreachable!()
            }
        };

        let eq_11_12 = |dim: Length| -> Length {
            let v_oc = v_oc.get::<kilovolt>();
            let dim = dim.get::<mm>();

            let y1 = dim - 660.4;
            let y2 = (v_oc + a) / b;
            let dim_1 = (660.4 + (y1 * y2)) / 25.4;
            Length::new::<inch>(dim_1)
        };

        // Table 6
        // The special case in this table appears to be height_1 for the VCBB configuration.

        // The actual conversion from mm to inch (according to `frink`) is:
        // mm -> inch = 5 / 127 (approx. 0.03937007874015748)
        //
        // However to match the printed text of IEEE 1584-2018, we must use the factor 0.03937 that is printed in the
        // standard.
        const MM_TO_IN: f64 = 0.03937;

        let width = if w < Length::new::<mm>(508.0) {
            match enclosure_type {
                EnclosureType::Typical => Length::new::<inch>(20.0),
                EnclosureType::Shallow => Length::new::<inch>(MM_TO_IN * w.get::<mm>()),
            }
        } else if w > Length::new::<mm>(508.0) && w <= Length::new::<mm>(660.4) {
            Length::new::<inch>(MM_TO_IN * w.get::<mm>())
        } else if w > Length::new::<mm>(660.4) && w <= Length::new::<mm>(1244.6) {
            eq_11_12(w)
        } else if w > Length::new::<mm>(1244.6) {
            eq_11_12(Length::new::<mm>(1244.6))
        } else {
            unreachable!()
        };

        let height = if h < Length::new::<mm>(508.0) {
            match enclosure_type {
                EnclosureType::Typical => Length::new::<inch>(20.0),
                EnclosureType::Shallow => Length::new::<inch>(MM_TO_IN * h.get::<mm>()),
            }
        } else if h > Length::new::<mm>(508.0) && h <= Length::new::<mm>(660.4) {
            Length::new::<inch>(MM_TO_IN * h.get::<mm>())
        } else if h > Length::new::<mm>(660.4) && h <= Length::new::<mm>(1244.6) {
            match ec {
                ElectrodeConfiguration::VCB => Length::new::<inch>(MM_TO_IN * h.get::<mm>()),
                ElectrodeConfiguration::VCBB | ElectrodeConfiguration::HCB => eq_11_12(h),
                _ => {
                    unreachable!()
                }
            }
        } else if h > Length::new::<mm>(1244.6) {
            match ec {
                ElectrodeConfiguration::VCB => Length::new::<inch>(49.0),
                ElectrodeConfiguration::VCBB | ElectrodeConfiguration::HCB => {
                    eq_11_12(Length::new::<mm>(1244.6))
                }
                _ => {
                    unreachable!()
                }
            }
        } else {
            unreachable!()
        };

        // Equation 13
        // let EES = ((height_1 + width_1) / 2.0).to(inch);
        let ees = (height + width) / 2.0;
        if enclosure_type == EnclosureType::Typical {
            // "For typical box enclosures, the minimum value of EES is 20."
            // Relax the criteria from ">= 20" to ">= 19.999" to allow for the imprecise conversion factor
            // of 1 mm = 0.03937 inch that is printed in the text of IEEE 1584-2018.
            debug_assert!(ees >= Length::new::<inch>(19.999))
        }
        // save calculation details for unit test purposes
        // self.height_1 = height_1
        // self.width_1 = width_1
        // self.EES = EES

        // Equation 14 / 15
        // key = (self.enclosure_type, self.EC)
        let b = TABLE_7.get(&(enclosure_type, ec)).unwrap();
        let ees = ees.get::<inch>();
        let x1 = b.b1 * ees.powi(2) + b.b2 * ees + b.b3;

        let cf = match enclosure_type {
            EnclosureType::Typical => x1,
            EnclosureType::Shallow => 1.0 / x1,
        };

        (cf, Some(EnclosureDebug { height, width, ees }))
    }
}

pub struct BoxDimensions {
    pub height: Length,
    pub width: Length,
    pub depth: Length,
}

impl Display for Cubicle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cubicle parameters:

V_oc (nominal voltage)          = {}
D (working distance)            = {}
G (busbar gap)                  = {}
EC (electrode configuration)    = {:?}

Box dimensions:
    height  = {}
    width   = {}
    depth   = {}",
            self.v_oc.into_format_args(kilovolt, Abbreviation),
            self.d.into_format_args(mm, Abbreviation),
            self.g.into_format_args(mm, Abbreviation),
            self.ec,
            self.dim.height.into_format_args(mm, Abbreviation),
            self.dim.width.into_format_args(mm, Abbreviation),
            self.dim.depth.into_format_args(mm, Abbreviation),
        )?;

        if let Some(debug) = &self.debug {
            write!(
                f,
                "

Enclosure correction factor
    enclosure_type  = {:?}
    height          = {}
    width           = {}
    EES             = {} in
    CF              = {}",
                self.enclosure_type,
                debug.height.into_format_args(mm, Abbreviation),
                debug.width.into_format_args(mm, Abbreviation),
                debug.ees,
                self.cf
            )?;
        }

        Ok(())
    }
}
