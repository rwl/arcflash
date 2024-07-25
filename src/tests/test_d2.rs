use crate::{e_afb, i_arc, Cubicle, EAfb, ElectrodeConfiguration, IArc};
use float_cmp::assert_approx_eq;
use uom::si::electric_current::{ampere, kiloampere, ElectricCurrent};
use uom::si::electric_potential::{kilovolt, volt, ElectricPotential};
use uom::si::length::{inch, kilometer, meter, micrometer, millimeter, Length};
use uom::si::radiant_exposure::joule_per_square_centimeter;
use uom::si::time::{microsecond, millisecond, second, Time};

/// Test low voltage calculation example, from Annex D.2.
#[test]
fn test_annex_d2_calc() {
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
    let t_arc_max = Time::new::<millisecond>(61.3);
    let t_arc_min = Time::new::<millisecond>(319.0);

    // Do all calculations
    let i_arc_max = i_arc(&cubicle, i_bf, false).unwrap();
    let e_afb_max = e_afb(&cubicle, i_arc_max.clone(), t_arc_max);

    let i_arc_min = i_arc(&cubicle, i_bf, true).unwrap();
    let e_afb_min = e_afb(&cubicle, i_arc_min.clone(), t_arc_min);

    assert_annex_d2(cubicle, i_arc_max, e_afb_max, i_arc_min, e_afb_min)
}

/// Test low voltage calculation example, from Annex D.2.
///
/// This is exactly the same as `test_annex_d2_calc` above, but with the input units changed.
/// The intent of this test is to ensure that units are correctly converted (by `uom`) throughout the code.
#[test]
fn test_annex_d2_calc_unit_conversion() {
    let cubicle = Cubicle::new(
        ElectricPotential::new::<volt>(480.0),
        ElectrodeConfiguration::VCB,
        Length::new::<meter>(0.032),
        Length::new::<kilometer>(0.0006096),
        Length::new::<micrometer>(610000.0),
        Length::new::<micrometer>(610000.0),
        Length::new::<micrometer>(254000.0),
    )
    .unwrap();

    let i_bf = ElectricCurrent::new::<ampere>(45000.0);
    let t_arc_max = Time::new::<second>(0.0613);
    let t_arc_min = Time::new::<microsecond>(319000.0);

    // Do all calculations
    let i_arc_max = i_arc(&cubicle, i_bf, false).unwrap();
    let e_afb_max = e_afb(&cubicle, i_arc_max.clone(), t_arc_max);

    let i_arc_min = i_arc(&cubicle, i_bf, true).unwrap();
    let e_afb_min = e_afb(&cubicle, i_arc_min.clone(), t_arc_min);

    assert_annex_d2(cubicle, i_arc_max, e_afb_max, i_arc_min, e_afb_min)
}

fn assert_annex_d2(
    cubicle: Cubicle,
    i_arc_max: IArc,
    e_afb_max: EAfb,
    i_arc_min: IArc,
    e_afb_min: EAfb,
) {
    // Step 1 //

    // D.82
    assert_approx_eq!(
        f64,
        i_arc_max.lv().i_arc_600.get::<kiloampere>(),
        32.449,
        epsilon = 1e-3
    );

    // Step 2 //

    // D.84
    assert_approx_eq!(
        f64,
        i_arc_max.lv().i_arc.get::<kiloampere>(),
        28.793,
        epsilon = 1e-3
    );

    // Step 3 //

    let debug = cubicle.debug.as_ref().unwrap();
    // D.86
    assert_approx_eq!(f64, debug.width.get::<inch>(), 24.016, epsilon = 1e-3);
    // D.87
    assert_approx_eq!(f64, debug.height.get::<inch>(), 24.016, epsilon = 1e-3);
    // D.88
    assert_approx_eq!(f64, debug.ees, 24.016, epsilon = 1e-3);
    // D.89
    assert_approx_eq!(f64, cubicle.cf, 1.085, epsilon = 1e-3);

    // Step 4 / Step 5 //

    // D.91
    assert_approx_eq!(
        f64,
        e_afb_max.lv().e.get::<joule_per_square_centimeter>(),
        11.585,
        epsilon = 1e-3
    );

    // Step 6 / Step 7 //

    // D.95
    assert_approx_eq!(
        f64,
        e_afb_max.lv().afb.get::<millimeter>(),
        1029.0,
        epsilon = 1e0
    );

    // Step 8 //
    // D.96
    assert_approx_eq!(f64, cubicle.var_cf, 0.247, epsilon = 1e-3);
    // D.97
    assert_approx_eq!(f64, 1.0 - 0.5 * cubicle.var_cf, 0.877, epsilon = 1e-3);

    // Step 9 //

    // D.99
    assert_approx_eq!(
        f64,
        i_arc_min.lv().i_arc.get::<kiloampere>(),
        25.244,
        epsilon = 1e-3
    );

    // Step 10 / Step 11 //

    // D.103
    assert_approx_eq!(
        f64,
        e_afb_min.lv().e.get::<joule_per_square_centimeter>(),
        53.156,
        epsilon = 1e-3
    );

    // Step 12 / Step 13 //

    // D.72
    assert_approx_eq!(
        f64,
        e_afb_min.lv().afb.get::<millimeter>(),
        2669.0,
        epsilon = 1e0
    );
}
