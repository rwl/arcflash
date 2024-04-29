// Copyright 2022, Li-aung Yip - https://www.penwatch.net
// Licensed under the MIT License. Refer LICENSE.txt.

use uom::si::electric_current::kiloampere;
use uom::si::electric_potential::kilovolt;
use uom::si::f64::{ElectricCurrent, Length, RadiantExposure, Time};
use uom::si::length::millimeter as mm;
use uom::si::radiant_exposure::joule_per_square_centimeter;
use uom::si::time::millisecond as ms;

use crate::common::NominalVoltage;
use crate::cubicle::Cubicle;
use crate::tables::{Table3_4_5Row, TABLE_1, TABLE_3, TABLE_4, TABLE_5};

// Equation 1
pub fn i_arc_intermediate(
    c: &Cubicle,
    v_oc: NominalVoltage,
    i_bf: ElectricCurrent,
) -> ElectricCurrent {
    // assert v_oc.check('[electric_potential]')
    // assert i_bf.check('[current]')

    // assert v_oc in (0.6 * kV, 2.7 * kV, 14.3 * kV,)

    // let _V_oc = v_oc.as_kv();
    let i_bf = i_bf.get::<kiloampere>();
    let g = c.g.get::<mm>();

    let k = TABLE_1.get(&(c.ec, v_oc)).unwrap();

    let x1 = k.k1 + k.k2 * f64::log10(i_bf) + k.k3 * f64::log10(g);

    let x2 = k.k4 * i_bf.powi(6)
        + k.k5 * i_bf.powi(5)
        + k.k6 * i_bf.powi(4)
        + k.k7 * i_bf.powi(3)
        + k.k8 * i_bf.powi(2)
        + k.k9 * i_bf.powi(1)
        + k.k10;

    let i_a = 10.0_f64.powf(x1) * x2;

    ElectricCurrent::new::<kiloampere>(i_a)
}

// Equation 2
pub fn i_arc_min(c: &Cubicle, i_arc: ElectricCurrent) -> ElectricCurrent {
    // assert i_arc.check('[current]')
    let i_arc = i_arc.get::<kiloampere>();

    ElectricCurrent::new::<kiloampere>(i_arc * (1.0 - 0.5 * c.var_cf))
}

pub fn intermediate_e(
    c: &Cubicle,
    v_oc: NominalVoltage,
    i_arc: ElectricCurrent,
    i_bf: ElectricCurrent,
    t: Time,
    i_arc_600: Option<ElectricCurrent>,
) -> RadiantExposure {
    // Implements equations 3, 4, 5, 6 for "intermediate incident energy".

    // assert V_oc.check('[electric_potential]')
    // assert I_arc.check('[current]')
    // assert I_bf.check('[current]')
    // assert T.check('[time]')
    // assert I_arc_600 is None or I_arc_600.check('[current]')

    // let _V_oc = V_oc.get::<kilovolt>();
    let i_arc: f64 = i_arc.get::<kiloampere>();
    let i_bf: f64 = i_bf.get::<kiloampere>();
    let t: f64 = t.get::<ms>();
    let i_arc_600: Option<f64> = if i_arc_600.is_none() {
        None
    } else {
        Some(i_arc_600.unwrap().get::<kiloampere>())
    };
    let g = c.g.get::<mm>();
    let cf = c.cf;
    let d = c.d.get::<mm>();

    // assert (V_oc <= 0.6 * kV) or (V_oc in (0.6 * kV, 2.7 * kV, 14.3 * kV,))

    let k: &Table3_4_5Row = match v_oc {
        NominalVoltage::V0_6 => TABLE_3.get(&c.ec).unwrap(),
        NominalVoltage::V2_7 => TABLE_4.get(&c.ec).unwrap(),
        NominalVoltage::V14_3 => TABLE_5.get(&c.ec).unwrap(),
    };
    // let k = if V_oc <= 0.6 * kV {
    //     TABLE_3[c.EC]
    // } else if V_oc == 2.7 * kV {
    //     TABLE_4[c.EC]
    // } else if V_oc == 14.3 * kV {
    //     TABLE_5[c.EC]
    // } else {
    //     None
    // };

    let x1 = 12.552 / 50.0 * t;

    let x2 = k.k1 + k.k2 * f64::log10(g);

    let x3_num = if let Some(i_arc_600) = i_arc_600 {
        // LV case. Eq 6.
        k.k3 * i_arc_600
    } else {
        // HV case. Eqs 3, 4, 5
        k.k3 * i_arc
    };

    let x3_den = k.k4 * i_bf.powi(7)
        + k.k5 * i_bf.powi(6)
        + k.k6 * i_bf.powi(5)
        + k.k7 * i_bf.powi(4)
        + k.k8 * i_bf.powi(3)
        + k.k9 * i_bf.powi(2)
        + k.k10 * i_bf;

    let x3 = x3_num / x3_den;

    let x4 = k.k11 * f64::log10(i_bf) + k.k13 * f64::log10(i_arc) + f64::log10(1.0 / cf);

    let x5 = k.k12 * f64::log10(d);

    // Equations 3, 4, 5, 6
    let e = x1 * 10.0_f64.powf(x2 + x3 + x4 + x5);

    // assert E >= 0

    RadiantExposure::new::<joule_per_square_centimeter>(e) // * J_per_sq_cm
}

// Implements equations 7, 8, 9, 10, for "intermediate arc flash boundary", in a simpler way.
//
// Calculates the (intermediate) arc flash boundary, i.e. AFB_600, from the incident energy i.e. E_600 only.
// Knowledge of T, G, I_arc, I_bf, and CF is not required, as it would be if using Eq's 7, 8, 9, 10 directly.
// This is useful for multi-time-step calculations where there is no singular value of T, I_arc, or I_bf.
//
// Motivation:
// ===========
//
// The IEEE 1584-2018 formulas for arc flash boundary (AFB), i.e. eq's 7, 8, 9, and 10, are pretty complicated.
//
// In particular, the equations for AFB requires knowledge of time T, busbar gap G, the currents I_arc and I_bf,
// and size correction factor CF.
//
// This is a problem when doing multi-time-step arc flash calculations where the values of T, I_arc, and I_bf are
// different for each time-step. What single value of I_arc would you plug into Eq 7, when I_arc is 10 kA for 100 ms,
// then 5 kA for 900 ms, then 2 kA for 1,000 ms?
//
// Details:
// ========
//
// Consider Eq 3 for the quantity E_600.
//
// First, we recognise that the relationship between incident energy E_600 (J/cm²) and distance D (mm) is simply
// that __the energy E_600 falls off exponentially with distance D__. If we rearrange Eq 3 using exponent identities
// we can simplify to:
//
//       E_600 = F * ( D ^ k12 )
//
// Where:
//   * E_600 is the __intermediate__ arcing energy at V = 0.6 kV, with units of J/cm²,
//   * F_600 is a (reasonably complicated) function of time T, busbar gap G, currents I_arc and I_bf, and size
//     correction factor CF,
//   * D is the distance (mm) from the fault to where E_600 has been measured.
//   * k12 is a constant __"distance exponent"__ from Table 3, Table 4, or Table 5.
//
// We can calculate what F_600** would have been:
//
//       F_600 = E_600 / ( D ^ k12 )
//
// Once we know the value of F_600, we can calculate E_600' at any distance D' we like:
//
//       E_600' (at distance D') = F_600 * ( D' ^ k12 )
//
// Alternately, we can calculate the distance D' that will give a particular value of E_600'.
//
//       D' = (E_600' / F_600) ^ ( 1 / k12 )
//
// Finally note that the arc flash boundary, AFB_600, is simply the special case where E_600' is equal to exactly
// 1.2 cal/cm². Noting that 1.2 cal/cm² * 4.184 J/cal = 5.0208 J/cm²,
//
//       AFB_600 = (5.0208 / F_600) ^ ( 1 / k12 )
//
// ** Sidenote: The physical meaning of the quantity "F_600" is that F_600 is, in some sense, the total amount of
// energy released (i.e. Joules). (The distribution of energy is not isotropic, i.e. k12 != -2.00, so this
// interpretation is not exact.)
//
// Sidenote 2: the funny number "50/12.552" in Eq 3/4/5/6 turns into the magic number 20 in Eq 7/8/9/10.
// 1.2 cal/cm² × 4.184 J/cal = 5.0208 J/cm²
// 50 / 12.552 * 5.0208 = 20 (exact)
pub fn intermediate_afb_from_e(c: &Cubicle, v_oc: NominalVoltage, e: RadiantExposure) -> Length {
    // assert V_oc.check('[electric_potential]')
    // assert E.check('[energy]/[area]')

    // _V_oc = V_oc.m_as(kV)
    let e = e.get::<joule_per_square_centimeter>();
    let d = c.d.get::<mm>();

    // assert (V_oc <= 0.6 * kV) or (V_oc in (0.6 * kV, 2.7 * kV, 14.3 * kV,))

    // if V_oc <= 0.6 * kV:
    //     k = table_3[c.EC]
    // elif V_oc == 2.7 * kV:
    //     k = table_4[c.EC]
    // elif V_oc == 14.3 * kV:
    //     k = table_5[c.EC]
    // else:
    //     k = None
    let k: &Table3_4_5Row = match v_oc {
        NominalVoltage::V0_6 => TABLE_3.get(&c.ec).unwrap(),
        NominalVoltage::V2_7 => TABLE_4.get(&c.ec).unwrap(),
        NominalVoltage::V14_3 => TABLE_5.get(&c.ec).unwrap(),
    };

    // After all the explanation, calculation of the (intermediate) AFB is simply 2 lines.
    let f = e / d.powf(k.k12);
    let afb = (5.0208 / f).powf(1.0 / k.k12);
    // assert AFB >= 0
    Length::new::<mm>(afb)
}

pub fn interpolate_e(
    c: &Cubicle,
    x_600: RadiantExposure,
    x_2700: RadiantExposure,
    x_14300: RadiantExposure,
) -> RadiantExposure {
    let v_oc = c.v_oc.get::<kilovolt>();

    // Eq 16, Eq 19, Eq 22
    let x1 = (((x_2700 - x_600) / 2.1) * (v_oc - 2.7)) + x_2700;
    // Eq 17, Eq 20, Eq 23
    let x2 = (((x_14300 - x_2700) / 11.6) * (v_oc - 14.3)) + x_14300;
    // Eq 18, Eq 21, Eq 24
    let x3 = ((x1 * (2.7 - v_oc)) / 2.1) + ((x2 * (v_oc - 0.6)) / 2.1);

    if 0.600 < v_oc && v_oc <= 2.7 {
        x3
    } else if v_oc > 2.7 {
        x2
    } else {
        unreachable!()
    }
}

pub fn interpolate_l(c: &Cubicle, x_600: Length, x_2700: Length, x_14300: Length) -> Length {
    let v_oc = c.v_oc.get::<kilovolt>();

    // Eq 16, Eq 19, Eq 22
    let x1 = (((x_2700 - x_600) / 2.1) * (v_oc - 2.7)) + x_2700;
    // Eq 17, Eq 20, Eq 23
    let x2 = (((x_14300 - x_2700) / 11.6) * (v_oc - 14.3)) + x_14300;
    // Eq 18, Eq 21, Eq 24
    let x3 = ((x1 * (2.7 - v_oc)) / 2.1) + ((x2 * (v_oc - 0.6)) / 2.1);

    if 0.600 < v_oc && v_oc <= 2.7 {
        x3
    } else if v_oc > 2.7 {
        x2
    } else {
        unreachable!()
    }
}

pub fn interpolate_c(
    c: &Cubicle,
    x_600: ElectricCurrent,
    x_2700: ElectricCurrent,
    x_14300: ElectricCurrent,
) -> ElectricCurrent {
    let v_oc = c.v_oc.get::<kilovolt>();

    // Eq 16, Eq 19, Eq 22
    let x1 = (((x_2700 - x_600) / 2.1) * (v_oc - 2.7)) + x_2700;
    // Eq 17, Eq 20, Eq 23
    let x2 = (((x_14300 - x_2700) / 11.6) * (v_oc - 14.3)) + x_14300;
    // Eq 18, Eq 21, Eq 24
    let x3 = ((x1 * (2.7 - v_oc)) / 2.1) + ((x2 * (v_oc - 0.6)) / 2.1);

    if 0.600 < v_oc && v_oc <= 2.7 {
        x3
    } else if v_oc > 2.7 {
        x2
    } else {
        unreachable!()
    }
}

// pub fn interpolate<K: Sub<Output = f64> + Copy>(c: &Cubicle, x_600: K, x_2700: K, x_14300: K) -> K {
//     let v_oc = c.v_oc.get::<kilovolt>();
//
//     // Eq 16, Eq 19, Eq 22
//     let x1 = (((x_2700 - x_600) / 2.1) * (v_oc - 2.7)) + x_2700;
//     // Eq 17, Eq 20, Eq 23
//     let x2 = (((x_14300 - x_2700) / 11.6) * (v_oc - 14.3)) + x_14300;
//     // Eq 18, Eq 21, Eq 24
//     let x3 = ((x1 * (2.7 - v_oc)) / 2.1) + ((x2 * (v_oc - 0.6)) / 2.1);
//
//     if 0.600 < v_oc && v_oc <= 2.7 {
//         x3
//     } else if v_oc > 2.7 {
//         x2
//     } else {
//         unreachable!()
//     }
// }

// Equation 25
pub fn i_arc_final_lv(
    c: &Cubicle,
    i_arc_600: ElectricCurrent,
    i_bf: ElectricCurrent,
) -> ElectricCurrent {
    // assert I_arc_600.check('[current]')
    // assert I_bf.check('[current]')

    let v_oc = c.v_oc.get::<kilovolt>();
    let i_arc_600 = i_arc_600.get::<kiloampere>();
    let i_bf = i_bf.get::<kiloampere>();

    let x1 = (0.6 / v_oc).powi(2);
    let x2 = 1.0 / i_arc_600.powi(2);
    let x3 = (0.6_f64.powi(2) - v_oc.powi(2)) / (0.6_f64.powi(2) * i_bf.powi(2));
    let x4 = f64::sqrt(x1 * (x2 - x3));

    ElectricCurrent::new::<kiloampere>(1.0 / x4)
}
