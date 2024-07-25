use crate::{e_afb, i_arc, multistep_e_and_afb, Cubicle, ElectrodeConfiguration};
use float_cmp::assert_approx_eq;
use uom::si::electric_current::{kiloampere, ElectricCurrent};
use uom::si::electric_potential::{kilovolt, ElectricPotential};
use uom::si::length::{millimeter, Length};
use uom::si::radiant_exposure::joule_per_square_centimeter;
use uom::si::time::{millisecond, Time};

/// Test high voltage calculation example, from Annex D.1.
///
/// Test that doing a calculation in 2 steps gives an identical result to doing the calculation in 1 step.
#[test]
fn test_annex_d1_calc_multistep() {
    let cubicle = Cubicle::new(
        ElectricPotential::new::<kilovolt>(4.16),
        ElectrodeConfiguration::VCB,
        Length::new::<millimeter>(104.0),
        Length::new::<millimeter>(914.4),
        Length::new::<millimeter>(1143.0),
        Length::new::<millimeter>(762.0),
        Length::new::<millimeter>(508.0),
    )
    .unwrap();
    let i_bf = ElectricCurrent::new::<kiloampere>(15.0);

    // Total 197ms as per original example
    let t_arc_max_1 = Time::new::<millisecond>(170.0);
    let t_arc_max_2 = Time::new::<millisecond>(27.0);

    // Total 223 ms as per original example
    let t_arc_min_1 = Time::new::<millisecond>(200.0);
    let t_arc_min_2 = Time::new::<millisecond>(23.0);

    // Do all calculations

    let i_arc_max_1 = i_arc(&cubicle, i_bf, false).unwrap();
    let e_afb_max_1 = e_afb(&cubicle, i_arc_max_1.clone(), t_arc_max_1);

    let i_arc_max_2 = i_arc(&cubicle, i_bf, false).unwrap();
    let e_afb_max_2 = e_afb(&cubicle, i_arc_max_2.clone(), t_arc_max_2);

    let (max_e, max_afb) = multistep_e_and_afb(&cubicle, &[e_afb_max_1, e_afb_max_2]);

    let i_arc_min_1 = i_arc(&cubicle, i_bf, true).unwrap();
    let e_afb_min_1 = e_afb(&cubicle, i_arc_min_1.clone(), t_arc_min_1);

    let i_arc_min_2 = i_arc(&cubicle, i_bf, true).unwrap();
    let e_afb_min_2 = e_afb(&cubicle, i_arc_min_2.clone(), t_arc_min_2);

    let (min_e, min_afb) = multistep_e_and_afb(&cubicle, &[e_afb_min_1, e_afb_min_2]);

    // Step 5 //

    // D.32
    assert_approx_eq!(
        f64,
        max_e.get::<joule_per_square_centimeter>(),
        12.152,
        epsilon = 1e-3
    );

    // Step 7 //

    // D.42
    assert_approx_eq!(f64, max_afb.get::<millimeter>(), 1606.0, epsilon = 1e0);

    // Step 12 //

    // D.62
    assert_approx_eq!(
        f64,
        min_e.get::<joule_per_square_centimeter>(),
        13.343,
        epsilon = 1e-3
    );

    // Step 14 //

    // D.72
    assert_approx_eq!(f64, min_afb.get::<millimeter>(), 1704.0, epsilon = 1e0);
}

/// Test low voltage calculation example, from Annex D.2.
///
/// Test that doing a calculation in 2 steps gives an identical result to doing the calculation in 1 step.
#[test]
fn test_annex_d2_calc_multistep() {
    let cubicle = Cubicle::new(
        ElectricPotential::new::<kilovolt>(0.48),
        ElectrodeConfiguration::VCB,
        Length::new::<millimeter>(32.0),
        Length::new::<millimeter>(609.6),
        Length::new::<millimeter>(610.0),
        Length::new::<millimeter>(610.0),
        Length::new::<millimeter>(254.0),
    )
    .unwrap();
    let i_bf = ElectricCurrent::new::<kiloampere>(45.0);

    // Original 61.3 ms
    let t_arc_max_1 = Time::new::<millisecond>(50.0);
    let t_arc_max_2 = Time::new::<millisecond>(11.3);

    // Original 319 ms
    let t_arc_min_1 = Time::new::<millisecond>(200.0);
    let t_arc_min_2 = Time::new::<millisecond>(119.0);

    // Do all calculations

    let i_arc_max_1 = i_arc(&cubicle, i_bf, false).unwrap();
    let e_afb_max_1 = e_afb(&cubicle, i_arc_max_1.clone(), t_arc_max_1);

    let i_arc_max_2 = i_arc(&cubicle, i_bf, false).unwrap();
    let e_afb_max_2 = e_afb(&cubicle, i_arc_max_2.clone(), t_arc_max_2);

    let (max_e, max_afb) = multistep_e_and_afb(&cubicle, &[e_afb_max_1, e_afb_max_2]);

    let i_arc_min_1 = i_arc(&cubicle, i_bf, true).unwrap();
    let e_afb_min_1 = e_afb(&cubicle, i_arc_min_1.clone(), t_arc_min_1);

    let i_arc_min_2 = i_arc(&cubicle, i_bf, true).unwrap();
    let e_afb_min_2 = e_afb(&cubicle, i_arc_min_2.clone(), t_arc_min_2);

    let (min_e, min_afb) = multistep_e_and_afb(&cubicle, &[e_afb_min_1, e_afb_min_2]);

    // Step 4 / Step 5 //

    // D.91
    assert_approx_eq!(
        f64,
        max_e.get::<joule_per_square_centimeter>(),
        11.585,
        epsilon = 1e-3
    );

    // Step 6 / Step 7 //

    // D.95
    assert_approx_eq!(f64, max_afb.get::<millimeter>(), 1029.0, epsilon = 1e0);

    // Step 10 / Step 11 //

    // D.103
    assert_approx_eq!(
        f64,
        min_e.get::<joule_per_square_centimeter>(),
        53.156,
        epsilon = 1e-3
    );

    // Step 12 / Step 13 //

    // D.106
    assert_approx_eq!(f64, min_afb.get::<millimeter>(), 2669.0, epsilon = 1e0);
}
