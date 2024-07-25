use crate::{e_afb, i_arc, Cubicle, EAfb, ElectrodeConfiguration, IArc};
use float_cmp::assert_approx_eq;
use uom::si::electric_current::{kiloampere, milliampere, ElectricCurrent};
use uom::si::electric_potential::{kilovolt, millivolt, ElectricPotential};
use uom::si::length::{inch, kilometer, meter, micrometer, millimeter, Length};
use uom::si::radiant_exposure::joule_per_square_centimeter;
use uom::si::time::{microsecond, millisecond, second, Time};

/// Test high voltage calculation example, from Annex D.1.
#[test]
fn test_annex_d1_calc() {
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
    let t_arc_max = Time::new::<millisecond>(197.0);
    let t_arc_min = Time::new::<millisecond>(223.0);

    // Do all calculations
    let i_arc_max = i_arc(&cubicle, i_bf, false).unwrap();
    let e_afb_max = e_afb(&cubicle, i_arc_max.clone(), t_arc_max);

    let i_arc_min = i_arc(&cubicle, i_bf, true).unwrap();
    let e_afb_min = e_afb(&cubicle, i_arc_min.clone(), t_arc_min);

    assert_annex_d1(cubicle, i_arc_max, e_afb_max, i_arc_min, e_afb_min)
}

/// Test high voltage calculation example, from Annex D.1.
///
/// This is exactly the same as `test_annex_d1_calc` above, but with the input units changed.
/// The intent of this test is to ensure that units are correctly converted (by `uom`) throughout the code.
#[test]
fn test_annex_d1_calc_unit_conversion() {
    let cubicle = Cubicle::new(
        ElectricPotential::new::<millivolt>(4160000.0),
        ElectrodeConfiguration::VCB,
        Length::new::<meter>(0.104),
        Length::new::<micrometer>(914400.0),
        Length::new::<meter>(1.143),
        Length::new::<kilometer>(0.000762),
        Length::new::<micrometer>(508000.0),
    )
    .unwrap();
    let i_bf = ElectricCurrent::new::<milliampere>(15000000.0);
    let t_arc_max = Time::new::<second>(0.197);
    let t_arc_min = Time::new::<microsecond>(223000.0);

    // Do all calculations

    let i_arc_max = i_arc(&cubicle, i_bf, false).unwrap();
    let e_afb_max = e_afb(&cubicle, i_arc_max.clone(), t_arc_max);

    let i_arc_min = i_arc(&cubicle, i_bf, true).unwrap();
    let e_afb_min = e_afb(&cubicle, i_arc_min.clone(), t_arc_min);

    assert_annex_d1(cubicle, i_arc_max, e_afb_max, i_arc_min, e_afb_min)
}

fn assert_annex_d1(
    cubicle: Cubicle,
    i_arc_max: IArc,
    e_afb_max: EAfb,
    i_arc_min: IArc,
    e_afb_min: EAfb,
) {
    // Step 1 //

    // D.9
    assert_approx_eq!(
        f64,
        i_arc_max.hv().i_arc_600.get::<kiloampere>(),
        11.117,
        epsilon = 1e-3
    );
    // D.11
    assert_approx_eq!(
        f64,
        i_arc_max.hv().i_arc_2700.get::<kiloampere>(),
        12.816,
        epsilon = 1e-3
    );
    // D.13
    assert_approx_eq!(
        f64,
        i_arc_max.hv().i_arc_14300.get::<kiloampere>(),
        14.116,
        epsilon = 1e-3
    );

    // Step 2 //

    // D.17
    assert_approx_eq!(
        f64,
        i_arc_max.hv().i_arc.get::<kiloampere>(),
        12.979,
        epsilon = 1e-3
    );

    // Step 3 //

    let debug = cubicle.debug.as_ref().unwrap();
    // D.19
    assert_approx_eq!(f64, debug.width.get::<inch>(), 27.632, epsilon = 1e-3);
    // D.20
    assert_approx_eq!(f64, debug.height.get::<inch>(), 45.0, epsilon = 1e-3);
    // D.21
    assert_approx_eq!(f64, debug.ees, 36.316, epsilon = 1e-3);
    // D.22
    assert_approx_eq!(f64, cubicle.cf, 1.284, epsilon = 1e-3);

    // Step 4 //

    // D.24
    assert_approx_eq!(
        f64,
        e_afb_max.hv().e_600.get::<joule_per_square_centimeter>(),
        8.652,
        epsilon = 1e-3
    );
    // D.26
    assert_approx_eq!(
        f64,
        e_afb_max.hv().e_2700.get::<joule_per_square_centimeter>(),
        11.977,
        epsilon = 1e-3
    );
    // D.28
    assert_approx_eq!(
        f64,
        e_afb_max.hv().e_14300.get::<joule_per_square_centimeter>(),
        13.367,
        epsilon = 1e-3
    );

    // Step 5 //

    // D.32
    assert_approx_eq!(
        f64,
        e_afb_max.hv().e.get::<joule_per_square_centimeter>(),
        12.152,
        epsilon = 1e-3
    );

    // Step 6 //

    // D.34
    assert_approx_eq!(
        f64,
        e_afb_max.hv().afb_600.get::<millimeter>(),
        1285.0,
        epsilon = 1e0
    );
    // D.36
    assert_approx_eq!(
        f64,
        e_afb_max.hv().afb_2700.get::<millimeter>(),
        1591.0,
        epsilon = 1e0
    );
    // D.38
    assert_approx_eq!(
        f64,
        e_afb_max.hv().afb_14300.get::<millimeter>(),
        1707.0,
        epsilon = 1e0
    );

    // Step 7 //

    // D.42
    assert_approx_eq!(
        f64,
        e_afb_max.hv().afb.get::<millimeter>(),
        1606.0,
        epsilon = 1e0
    );

    // Step 8 //

    // D.43
    assert_approx_eq!(f64, cubicle.var_cf, 0.047, epsilon = 1e-3);
    // D.44
    assert_approx_eq!(f64, 1.0 - 0.5 * cubicle.var_cf, 0.977, epsilon = 1e-3);

    // Step 9 //

    // D.45
    assert_approx_eq!(
        f64,
        i_arc_min.hv().i_arc_600.get::<kiloampere>(),
        10.856,
        epsilon = 1e-3
    );
    // D.46
    assert_approx_eq!(
        f64,
        i_arc_min.hv().i_arc_2700.get::<kiloampere>(),
        12.515,
        epsilon = 1e-3
    );
    // D.47
    assert_approx_eq!(
        f64,
        i_arc_min.hv().i_arc_14300.get::<kiloampere>(),
        13.786,
        epsilon = 1e-3
    );

    // Step 10 //

    // D.51
    assert_approx_eq!(
        f64,
        i_arc_min.hv().i_arc.get::<kiloampere>(),
        12.675,
        epsilon = 1e-3
    );

    // Step 11 //

    // D.54
    assert_approx_eq!(
        f64,
        e_afb_min.hv().e_600.get::<joule_per_square_centimeter>(),
        8.980,
        epsilon = 1e-3
    );
    // D.56
    assert_approx_eq!(
        f64,
        e_afb_min.hv().e_2700.get::<joule_per_square_centimeter>(),
        13.018,
        epsilon = 1e-3
    );
    // D.58
    assert_approx_eq!(
        f64,
        e_afb_min.hv().e_14300.get::<joule_per_square_centimeter>(),
        15.602,
        epsilon = 1e-3
    );

    // Step 12 //

    // D.62
    assert_approx_eq!(
        f64,
        e_afb_min.hv().e.get::<joule_per_square_centimeter>(),
        13.343,
        epsilon = 1e-3
    );

    // Step 13 //

    // D.64
    assert_approx_eq!(
        f64,
        e_afb_min.hv().afb_600.get::<millimeter>(),
        1316.0,
        epsilon = 1e0
    );
    // D.66
    assert_approx_eq!(
        f64,
        e_afb_min.hv().afb_2700.get::<millimeter>(),
        1678.0,
        epsilon = 1e0
    );
    // D.68
    assert_approx_eq!(
        f64,
        e_afb_min.hv().afb_14300.get::<millimeter>(),
        1884.0,
        epsilon = 1e0
    );

    // Step 14 //

    // D.72
    assert_approx_eq!(
        f64,
        e_afb_min.hv().afb.get::<millimeter>(),
        1704.0,
        epsilon = 1e0
    );
}
