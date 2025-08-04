use serde::{Deserialize, Serialize};

/// Available countries for devices in the Mender server.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Country {
    /// Austria
    At,
    /// Belgium
    Be,
    /// Bulgaria
    Bg,
    /// Switzerland
    Ch,
    /// China
    Cn,
    /// Cyprus
    Cy,
    /// Czech Republic
    Cz,
    /// Germany
    De,
    /// Denmark
    Dk,
    /// Estonia
    Ee,
    /// Spain
    Es,
    /// Finland
    Fi,
    /// France
    Fr,
    /// Great Britain
    Gb,
    /// Greece
    Gr,
    /// Ireland
    Ie,
    /// Iceland
    Is,
    /// Hungary
    Hu,
    /// Italy
    It,
    /// Luxembourg
    Lu,
    /// Netherlands
    Nl,
    /// Norway
    No,
    /// Poland
    Pl,
    /// Portugal
    Pt,
    /// Romania
    Ro,
    /// Russia
    Ru,
    /// Serbia
    Rs,
    /// Sweden
    Se,
    /// Slovenia
    Si,
    /// Slovakia
    Sk,
    /// Croatia
    Hr,
    /// Latvia
    Lv,
    /// Lithuania
    Lt,
}
