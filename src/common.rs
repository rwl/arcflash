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
    V600,
    V2700,
    V14300,
}
