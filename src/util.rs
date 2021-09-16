use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Style {
    Default,
    Primary,
    Danger,
}

impl Default for Style {
    fn default() -> Self {
        Style::Default
    }
}
