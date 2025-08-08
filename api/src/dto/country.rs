use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Available countries for device in the Mender server.
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

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::At => write!(f, "AT"),
            Self::Be => write!(f, "BE"),
            Self::Bg => write!(f, "BG"),
            Self::Ch => write!(f, "CH"),
            Self::Cn => write!(f, "CN"),
            Self::Cy => write!(f, "CY"),
            Self::Cz => write!(f, "CZ"),
            Self::De => write!(f, "DE"),
            Self::Dk => write!(f, "DK"),
            Self::Ee => write!(f, "EE"),
            Self::Es => write!(f, "ES"),
            Self::Fi => write!(f, "FI"),
            Self::Fr => write!(f, "FR"),
            Self::Gb => write!(f, "GB"),
            Self::Gr => write!(f, "GR"),
            Self::Ie => write!(f, "IE"),
            Self::Is => write!(f, "IS"),
            Self::Hu => write!(f, "HU"),
            Self::It => write!(f, "IT"),
            Self::Lu => write!(f, "LU"),
            Self::Nl => write!(f, "NL"),
            Self::No => write!(f, "NO"),
            Self::Pl => write!(f, "PL"),
            Self::Pt => write!(f, "PT"),
            Self::Ro => write!(f, "RO"),
            Self::Ru => write!(f, "RU"),
            Self::Rs => write!(f, "RS"),
            Self::Se => write!(f, "SE"),
            Self::Si => write!(f, "SI"),
            Self::Sk => write!(f, "SK"),
            Self::Hr => write!(f, "HR"),
            Self::Lv => write!(f, "LV"),
            Self::Lt => write!(f, "LT"),
        }
    }
}
