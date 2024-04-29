#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum ElectrodeConfiguration {
    VCB,
    VCBB,
    HCB,
    HOA,
    VOA,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum EnclosureType {
    Typical,
    Shallow,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum NominalVoltage {
    V0_6,
    V2_7,
    V14_3,
}
