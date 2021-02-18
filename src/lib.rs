use serde::{Deserialize, Serialize};

pub mod blocks;
pub mod composition;
pub mod elements;

use composition::Text;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<blocks::Block>,

    pub thread_ts: Option<String>,

    #[serde(default = "default_mrkdwn")]
    pub mrkdwn: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Modal {
    pub title: Text,

    pub submit: Text,
    pub close: Text,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<blocks::Block>,
}

fn default_mrkdwn() -> bool {
    true
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Style {
    Default,
    Primary,
    Danger,
}
