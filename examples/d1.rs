use arcflash::{e_afb, i_arc, Cubicle, ElectrodeConfiguration};
use uom::si::electric_current::{kiloampere, ElectricCurrent};
use uom::si::electric_potential::{kilovolt, ElectricPotential};
use uom::si::length::{millimeter, Length};
use uom::si::time::{millisecond, Time};

/// IEEE 1584-2018 "D.1 Sample arc-flash incident energy calculation for a medium-voltage system"
fn main() {
    println!("Medium voltage example");

    let i_bf = ElectricCurrent::new::<kiloampere>(15.0);
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

    let i_arc_max = i_arc(&cubicle, i_bf, false).unwrap();
    let i_arc_min = i_arc(&cubicle, i_bf, true).unwrap();

    // Pass the values of i_arc_max and i_arc_min out to external software to determine clearing times T
    let t_arc_max = Time::new::<millisecond>(197.0);
    let t_arc_min = Time::new::<millisecond>(223.0);

    let e_afb_max = e_afb(&cubicle, i_arc_max, t_arc_max);
    let e_afb_min = e_afb(&cubicle, i_arc_min, t_arc_min);

    println!("{}", cubicle);
    println!("{}", i_arc_max);
    println!("{}", e_afb_max);
    println!("{}", i_arc_min);
    println!("{}", e_afb_min);

    if e_afb_max.e() > e_afb_min.e() {
        println!("The maximum arcing current case was highest energy.");
    } else {
        println!("The minimum arcing current case was highest energy.");
    }
}
