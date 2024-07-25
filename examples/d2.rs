use arcflash::{e_afb, i_arc, Cubicle, ElectrodeConfiguration};
use uom::si::electric_current::{kiloampere, ElectricCurrent};
use uom::si::electric_potential::{kilovolt, ElectricPotential};
use uom::si::length::{millimeter, Length};
use uom::si::time::{millisecond, Time};

/// IEEE 1584-2018 "D.2 Sample arc-flash incident energy calculation for a low-voltage system"
fn main() {
    println!("Low voltage example");

    let i_bf = ElectricCurrent::new::<kiloampere>(45.0);
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

    let i_arc_max = i_arc(&cubicle, i_bf, false).unwrap();
    let i_arc_min = i_arc(&cubicle, i_bf, true).unwrap();

    // Pass the values of i_arc_max and i_arc_min out to external software to determine clearing times T
    let t_arc_max = Time::new::<millisecond>(61.3);
    let t_arc_min = Time::new::<millisecond>(319.0);

    let e_afb_max = e_afb(&cubicle, i_arc_max.clone(), t_arc_max);
    let e_afb_min = e_afb(&cubicle, i_arc_min.clone(), t_arc_min);

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
