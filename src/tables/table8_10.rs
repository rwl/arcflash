use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TABLE_8_10: HashMap<&'static str, TypicalParams> = {
        HashMap::from([
            (
                "15kV Switchgear",
                TypicalParams::new(152.0, 1143.0, 762.0, 762.0, 914.4),
            ),
            (
                "15kV MCC, 152",
                TypicalParams::new(914.4, 914.4, 914.4, 914.4, 914.4),
            ),
            (
                "5kV Switchgear",
                TypicalParams::new(104.0, 914.4, 914.4, 914.4, 914.4),
            ),
            (
                "5kV Switchgear (2)",
                TypicalParams::new(104.0, 1143.0, 762.0, 762.0, 914.4),
            ),
            (
                "5kV MCC",
                TypicalParams::new(104.0, 660.4, 660.4, 660.4, 914.4),
            ),
            (
                "LV Switchgear",
                TypicalParams::new(32.0, 508.0, 508.0, 508.0, 609.6),
            ),
            (
                "LV MCC (Shallow)",
                TypicalParams::new(25.0, 355.6, 304.8, 100.0, 457.2),
            ),
            (
                "LV Panelboard (Shallow)",
                TypicalParams::new(25.0, 355.6, 304.8, 100.0, 457.2),
            ),
            (
                "LV MCC",
                TypicalParams::new(25.0, 355.6, 304.8, 250.0, 457.2),
            ),
            (
                "LV Panelboard",
                TypicalParams::new(25.0, 355.6, 304.8, 250.0, 457.2),
            ),
            (
                "Cable Junction Box (Shallow)",
                TypicalParams::new(13.0, 355.6, 304.8, 100.0, 457.2),
            ),
            (
                "Cable Junction Box",
                TypicalParams::new(13.0, 355.6, 304.8, 250.0, 457.2),
            ),
        ])
    };
}

/// This is a combination of table 8 and table 10.
/// Headers and equipment class names shortened for brevity.
/// G is typical busbar gap in mm.
/// bh, bw, and bd are enclosure (box) height, width, and depth in mm.
/// D is working distance in mm.
///
/// LV equipment with "shallow" depth <= 8 inches - set to 100 mm.
/// LV equipment with "deep" depth > 8 inches - set to 250 mm.
/// Precise depths don't matter, only whether the enclosure is "shallow" or "deep".
pub struct TypicalParams {
    /// Typical busbar gap in mm.
    pub g: f64,
    /// Enclosure (box) height in mm.
    pub bh: f64,
    /// Enclosure (box) width in mm.
    pub bw: f64,
    /// Enclosure (box) depth in mm.
    pub bd: f64,
    /// Working distance in mm.
    pub d: f64,
}

impl TypicalParams {
    fn new(g: f64, bh: f64, bw: f64, bd: f64, d: f64) -> Self {
        Self { g, bh, bw, bd, d }
    }
}
